from dataclasses import dataclass
import sys

ACOST = 3
BCOST = 1

class Machine:
    ax: int
    ay: int
    bx: int
    by: int
    prizex: int
    prizey: int

def calculate_colinear_cost(machine):
    # if button A and button B are co-linear, we pick whichever is the cheapest
    if machine.prizex / machine.ax % 1 != 0:
        return (machine.prizex / machine.bx) * BCOST
    elif machine.prizex / machine.bx % 1 != 0:
        return (machine.prizex / machine.ax) * ACOST
    else:
        return min((machine.prizex / machine.ax) * ACOST, (machine.prizex / machine.bx) * BCOST)
    return 0

# uses inverse matrix multiplication to calculate the prize in the button's coordinate space
def calculate_cost(machine):
    determinant = machine.ax * machine.by - machine.bx * machine.ay
    if determinant == 0:
        # print("determinant rejected")
        return calculate_colinear_cost(machine)
    
    a_presses = (machine.prizex * machine.by - machine.prizey * machine.bx) / determinant
    b_presses = (machine.prizex * -machine.ay + machine.prizey * machine.ax) / determinant
    # we can only press the buttons a whole number of times
    if a_presses % 1 != 0 or b_presses % 1 != 0:
        # print("whole number rejected")
        return 0
    # print(a_presses)
    # print(b_presses)
    
    return a_presses * ACOST + b_presses * BCOST


def parse_input(filename):
    machines_list = []
    with open(filename) as f:
        lines_read = 0
        cur_machine = Machine()
        for l in f.readlines():
            if lines_read == 0:
                line = l.strip().removeprefix("Button A: ").split(", ")
                cur_machine.ax = int(line[0].removeprefix("X+"))
                cur_machine.ay = int(line[1].removeprefix("Y+"))
            elif lines_read == 1:
                line = l.strip().removeprefix("Button B: ").split(", ")
                cur_machine.bx = int(line[0].removeprefix("X+"))
                cur_machine.by = int(line[1].removeprefix("Y+"))
            elif lines_read == 2:
                line = l.strip().removeprefix("Prize: ").split(", ")
                cur_machine.prizex = int(line[0].removeprefix("X="))
                cur_machine.prizey = int(line[1].removeprefix("Y="))
                machines_list.append(cur_machine)
                cur_machine = Machine()

            lines_read += 1
            if lines_read > 3:
                lines_read = 0
    return machines_list

# converts an input to be part 2
def complexify_input(machines_list):
    for i in range(len(machines_list)):
        machines_list[i].prizex += 10000000000000
        machines_list[i].prizey += 10000000000000

def main():
    input_1 = parse_input("input_1")
    cost_1 = sum(map(calculate_cost, input_1))
    print(cost_1) # should be 480
    input_3 = parse_input("input_3")
    cost_3 = sum(map(calculate_cost, input_3))
    print(cost_3) # should be 30
    input_2 = parse_input("input_2")
    cost_2 = sum(map(calculate_cost, input_2))
    print(cost_2) # should be 37901
    complexify_input(input_2)
    cost_2 = sum(map(calculate_cost, input_2))
    print(cost_2) # should be 77407675412647

if __name__ == "__main__":
    sys.exit(main())