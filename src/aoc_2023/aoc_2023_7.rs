use std::cmp::Ordering;

pub fn main() {
    println!("INPUT_1: {}", solve_simple(INPUT_1));
    println!("INPUT_2: {}", solve_simple(INPUT_2));
}

fn solve_simple(input: &str) -> i32 {
    let mut plays = process_input(input);
    plays.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    let (winnings, _) = plays
        .iter()
        .fold((0, 1), |(winnings, i), x| (winnings + i * x.bid, i + 1));
    winnings
}

fn solve_complex(input: &str) -> i32 {
    let mut plays = process_input(input);
    // So I don't have to rewrite process_input, go through and jokerfy all the hands after processing input.
    plays = plays
        .iter()
        .map(|play| Play {
            hand: Hand::jokerfy(play.hand.cards),
            bid: play.bid,
        })
        .collect();

    // Since hand type is correct based on the part two joker rules, and cards have been revalued
    // according to part two joker value rules, the sort and fold can be the same.
    plays.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    let (winnings, _) = plays
        .iter()
        .fold((0, 1), |(winnings, i), x| (winnings + i * x.bid, i + 1));
    winnings
}

fn process_input(input: &str) -> Vec<Play> {
    let input = input.trim();
    let mut plays = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut sides = line.split(' ');
        let (Some(hand), Some(bid)) = (sides.next(), sides.next()) else {
            panic!("less than 2 sides to line {}", i);
        };
        let cards = convert_hand(hand.chars().map(value_card).collect());
        let bid = bid.parse::<i32>().unwrap();

        plays.push(Play {
            hand: Hand::new(cards),
            bid,
        });
    }
    plays
}

// #[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Play {
    hand: Hand,
    bid: i32,
}

// TODO: change vector to HashMap
fn calculate_hand_type(cards: [i32; 5]) -> HandType {
    let mut unique_cards: Vec<(i32, i32)> = Vec::new();
    // Count unique cards in the hand.
    for card in cards {
        // For every card, either add to the unique card count, or add a new unique card to the vector.
        if unique_cards.iter().find(|(num, _)| *num == card).is_some() {
            for i in 0..unique_cards.len() {
                if unique_cards[i].0 == card {
                    unique_cards[i].1 = unique_cards[i].1 + 1;
                }
            }
        } else {
            unique_cards.push((card, 1));
        }
    }
    if unique_cards.len() == 1 {
        return HandType::Five;
    }
    if unique_cards.len() == 2 && unique_cards.iter().find(|(_, count)| *count == 4).is_some() {
        return HandType::Four;
    }
    if unique_cards.len() == 2 && unique_cards.iter().find(|(_, count)| *count == 3).is_some() {
        return HandType::Full;
    }
    if unique_cards.iter().find(|(_, count)| *count == 3).is_some() {
        return HandType::Three;
    }
    if unique_cards
        .iter()
        .filter(|(_, count)| *count == 2)
        .collect::<Vec<_>>()
        .len()
        == 2
    {
        return HandType::TwoPair;
    }
    if unique_cards.iter().find(|(_, count)| *count == 2).is_some() {
        return HandType::OnePair;
    }
    return HandType::High;
}

fn calculate_joker_hand_type(cards: [i32; 5]) -> HandType {
    HandType::High
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: [i32; 5],
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [i32; 5]) -> Hand {
        Hand {
            cards,
            hand_type: calculate_hand_type(cards),
        }
    }

    fn jokerfy(cards: [i32; 5]) -> Hand {
        let hand_type = calculate_joker_hand_type(cards);
        // reduce the joker cards for the complex solve
        let cards = cards.map(|i| if i == 9 { -1 } else { i });
        Hand {
            cards,
            hand_type: calculate_hand_type(cards),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type
    }
}

// TODO: change to match, and change the for loop to cmp, since that's how array cmp works anyway.
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self
            .hand_type
            .partial_cmp(&other.hand_type)
            .is_some_and(|result| result.is_ne())
        {
            return self.hand_type.partial_cmp(&other.hand_type);
        }
        for i in 0..self.cards.len() {
            if self.cards[i]
                .partial_cmp(&other.cards[i])
                .is_some_and(|result| result.is_ne())
            {
                return self.cards[i].partial_cmp(&other.cards[i]);
            }
        }
        Some(Ordering::Equal)
    }
}

fn value_card(c: char) -> i32 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("invalid character"),
    }
}

fn convert_hand(v: Vec<i32>) -> [i32; 5] {
    let mut output = [0; 5];
    if v.len() != 5 {
        panic!("improperly sized hand");
    }
    for (i, card) in v.iter().enumerate() {
        output[i] = *card;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // Puzzle tests
    #[test]
    fn solve_simple_input_1() {
        assert_eq!(solve_simple(INPUT_1), 6440);
    }

    #[test]
    fn solve_simple_input_2() {
        assert_eq!(solve_simple(INPUT_2), 246409899);
    }

    // Hand type parsing tests
    #[test]
    fn test_high() {
        assert_eq!(calculate_hand_type([3, 2, 1, 4, 5]), HandType::High);
    }

    #[test]
    fn test_one_pair() {
        assert_eq!(calculate_hand_type([3, 2, 1, 3, 4]), HandType::OnePair);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(calculate_hand_type([3, 4, 3, 2, 2]), HandType::TwoPair);
    }

    #[test]
    fn test_three_of_a_kind() {
        assert_eq!(calculate_hand_type([3, 3, 1, 3, 2]), HandType::Three);
    }

    #[test]
    fn test_full_house() {
        assert_eq!(calculate_hand_type([3, 1, 3, 1, 3]), HandType::Full);
    }

    #[test]
    fn test_four_of_a_kind() {
        assert_eq!(calculate_hand_type([3, 1, 3, 3, 3]), HandType::Four);
    }

    #[test]
    fn test_five_of_a_kind() {
        assert_eq!(calculate_hand_type([3, 3, 3, 3, 3]), HandType::Five);
    }

    // Hand tests

    #[test]
    fn hand_type_greater() {
        let hand1 = Hand::new([3, 1, 3, 3, 3]);
        let hand2 = Hand::new([3, 1, 1, 3, 3]);
        assert!(hand1 > hand2);
    }

    #[test]
    fn cards_greater() {
        let hand1 = Hand::new([3, 1, 3, 3, 3]);
        let hand2 = Hand::new([1, 3, 3, 3, 3]);
        assert!(hand1 > hand2);
    }

    #[test]
    fn hand_equal() {
        let hand1 = Hand::new([3, 1, 3, 3, 3]);
        assert_eq!(hand1, hand1);
    }
}

const INPUT_1: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

const INPUT_2: &str = r#"
342QK 491
36QAT 619
85663 606
33K3A 23
K7775 814
T67T6 105
49T8T 200
2KAT2 317
96669 251
4A827 285
QTQ6Q 308
A2T58 930
838T8 692
AAAKK 535
3T338 612
QAAQQ 526
55655 329
AQQQ2 621
KQQ88 190
97QQ8 870
8J833 494
6AJT8 318
AA4QQ 125
62KK6 876
7A7QK 241
TT2T2 385
43234 632
57798 393
5A4J8 623
93QA8 935
46KT2 288
37J73 503
7A55Q 668
2J368 525
36223 226
A6T36 291
2AJ8T 740
QQQ52 411
893A6 778
TA438 197
T2242 25
23TJ9 769
43Q32 782
77J35 659
JJA87 756
6T666 237
JAKAA 238
86348 869
A9959 671
2TA36 51
35Q5T 65
A8T6T 212
T5T5T 962
27722 893
5T962 562
4J463 338
QQ4JA 134
J2TK7 925
Q89K2 389
43463 349
99998 764
76QT5 683
T6733 92
QKKQQ 599
2TQ9Q 206
99T6Q 816
5434T 14
527Q5 466
3T384 944
5A927 907
TQTTQ 93
38377 425
96A99 864
JJJ28 304
88688 841
245KT 365
6T668 364
7AAQQ 586
JAJAA 973
24442 669
23432 293
QJ8T5 649
8K492 86
96446 278
JTT99 217
99292 381
4T666 803
79A9J 20
8A7A8 332
9948T 940
683TA 810
5J666 106
6J6Q6 972
JJJJJ 928
KK664 781
JJQTQ 957
TT49K 775
32337 707
A6Q53 548
K6K6K 703
T89KQ 1
95399 372
K8484 267
T3633 561
93868 136
TT23T 771
87QT3 547
AAK4A 465
2JJJK 966
3J48J 277
56Q82 266
AA6TA 87
73QTK 596
T2T22 988
A9999 948
Q2K6J 851
9KKK9 639
2Q987 824
29733 981
55Q73 723
KK222 378
22322 186
A9634 904
K67JT 558
TK329 856
77T77 736
A7J66 339
994AJ 798
9JK46 460
25286 58
TKTKK 761
3TA75 680
2TT9K 980
37376 284
KK23T 920
A5AAA 730
A5J65 551
TATTT 282
KTT44 324
33434 882
8K89K 479
82563 832
TTQQQ 990
A623K 780
JQQT7 565
T358Q 513
9298A 970
QQ7Q5 382
82826 863
7K23Q 340
QQ4Q5 590
66A66 699
44QA4 83
4TTTT 776
T5995 929
8Q78Q 786
222J4 380
9TK25 582
J6KA3 270
8888J 793
JJJ2J 209
Q5J7K 353
645QQ 445
54TTT 804
77747 898
22J52 515
3QQQ3 118
2443K 536
AQ8K5 55
TQAKJ 111
A96A3 688
54Q4K 676
JK546 315
J9J99 689
944T2 442
J3663 949
J9999 49
K7JAA 833
4AK4K 956
3A3A3 115
KKKKT 234
76K23 712
K82T6 463
J3729 15
8988J 187
AAA55 327
22K65 757
5Q55Q 744
88878 989
TKTTT 601
58855 746
T33J3 166
J9922 334
5Q3QQ 594
2Q2QQ 84
KJTKK 252
QA3AA 598
8QT8Q 609
52TA4 32
T66QT 992
TJAJT 215
J3227 743
2684Q 996
JKQQ8 36
8J558 351
98A78 233
T7TTT 54
QQ55J 927
483JK 648
QA99K 228
2TJA6 110
373JQ 732
TT24J 820
A83T7 654
95J8J 563
62866 313
6633K 726
J5556 881
J4464 967
4TQQT 985
422QA 222
8T88J 386
44JJ4 179
J66TT 410
96699 296
AA2AK 788
QQ396 176
7786K 917
3666J 477
Q6996 527
99444 41
2KT3A 675
KQKKQ 936
KQ493 437
7862J 502
T74A8 947
6J97A 801
66363 903
668Q8 352
T529T 330
26A6A 416
7J837 390
756A4 710
43Q68 33
66242 56
88Q88 868
7AAA7 273
AQ2AA 628
5Q522 938
23K36 216
38839 685
49J45 286
4774K 303
25Q55 915
849J7 57
42545 859
KA9AA 472
K4822 489
444K4 542
88Q55 538
99599 13
AAAK6 274
39636 354
933J3 568
78Q79 566
46664 402
TKTQ7 617
8676T 454
3KJ2K 650
8A585 999
2QQQQ 9
TT555 377
K2699 807
A5289 577
K7273 48
3QQQQ 818
KAKKK 152
93553 213
8QK2K 507
66J6J 202
TK5QT 931
84T57 844
QQ8JQ 960
28248 760
Q6987 890
JTAT5 496
99933 122
33KKT 729
A8349 815
K2369 645
93333 687
KKKK7 879
7727T 74
Q6K52 253
933Q7 462
748K9 597
AT5J9 384
83838 552
3TQ5K 774
899J3 260
777A7 301
95K26 713
JK3T5 711
45456 311
AKKA4 46
KK248 280
KJ755 878
69T8Q 653
Q966J 371
4K839 17
7T77K 556
7AQ44 362
6K626 986
7KQQQ 261
555T8 272
89984 229
4JA6K 320
J4494 627
J3444 310
62Q37 846
8K588 942
JQJQJ 147
5J876 952
KKJ89 717
KKKQ9 618
3TQ4J 663
7J5Q9 448
A7A73 862
793T2 979
377J6 73
3TAJJ 146
KTJ4Q 755
3K3K7 208
22J38 569
222QT 811
Q33JT 195
K666T 625
8QA47 295
J9J79 860
62AK2 221
7J755 417
37232 414
AATAA 470
TQQQQ 469
9344J 749
4333A 492
65576 767
K7677 160
Q5QQQ 434
KQ764 837
AQQQQ 580
AAA66 359
58923 735
J2J88 169
56T5K 60
J8T8T 595
3A8J9 982
A44AJ 900
KK77K 418
QT525 792
22282 250
Q84Q4 412
J5754 836
9JT3T 640
44A44 290
7AJAA 969
85AK2 85
25257 873
95959 604
KAJ8K 919
4AJA2 759
44A47 830
QQKQA 127
8Q8TJ 116
87783 543
JJ77J 300
K3Q5A 766
3TT3T 440
779Q4 916
42555 819
6JQ45 541
T2225 157
99JJ2 443
235A4 642
95455 559
8J865 660
K7K7J 512
6AA6Q 791
97977 341
9QQ99 456
6666Q 336
34Q3Q 433
A75T2 724
AJQ98 405
65A2J 509
A833J 420
29396 299
9954Q 438
J77J7 827
3333J 498
7KK9K 170
828TT 976
QT22T 120
KTK55 256
JAQ4T 858
888J3 264
7K58K 768
A9668 401
555A9 497
AA2AA 709
TT9TK 444
38T2J 370
AAAA8 138
9K4AT 693
9TAK8 275
Q684K 53
AATAT 933
2T32T 974
45444 581
88882 37
J9QQ7 388
43T33 537
AK729 823
AJ88A 828
225JT 510
4AJJA 21
72266 529
A8AAT 464
92222 843
A3J8Q 678
KTT83 100
86386 716
J4K59 690
J62K5 214
3KJAQ 842
K53A7 2
AJAAA 797
JAAKK 787
2QJ82 658
TT9T9 914
J4J4J 428
6465T 694
AK4A4 739
62484 398
AAKKK 636
6J576 167
AJA33 822
37K39 585
55959 149
9KA45 44
7JJ85 840
78882 344
J262Q 124
3496T 891
K4457 165
Q563T 977
J7882 159
A8899 519
3QJ28 734
6466J 587
TTJJT 355
87J7A 908
222QJ 103
7T762 883
4J444 995
TT3QT 248
TTQTT 451
8T53A 257
86649 331
KKA4K 185
85988 968
54555 958
T8T8T 705
T8JT9 909
4KK44 236
74735 239
88694 923
9J929 375
223T2 825
T7445 708
TJ63K 727
339Q9 452
28JJ4 408
7A33J 993
9T52Q 622
5K7A4 684
TJK44 289
2863A 544
65626 795
2A8J5 424
AAQQA 471
3K55K 750
77755 316
6AJ66 589
486T5 360
Q838T 395
32363 62
KAQAK 667
233K2 506
KK779 175
53A65 328
JK99K 892
TJTA2 276
9T9K9 204
63239 584
44222 714
K3K83 126
6J527 96
32333 861
KA247 913
J75QQ 436
82QJ9 885
89387 839
QAT39 109
2KTT7 545
33337 391
22227 560
6QQ44 848
29939 532
K6JKK 984
54484 108
4Q444 369
K88Q8 523
K55A5 624
85888 459
8Q8Q8 196
643T4 348
88J28 258
69999 88
8TT3T 486
2T56K 255
JT4T4 403
J8484 162
K88TA 82
JAJ9A 90
4K569 194
53535 481
TKJTK 367
48888 404
4T8TJ 34
QKA3T 476
252T5 620
7T77T 600
TJAAT 427
K3A97 143
46645 578
79497 570
TK9JK 68
J3J33 153
222A2 572
Q9999 243
3553J 983
4J277 630
TK4Q8 508
T44AQ 399
A3AA3 140
9QJ85 611
J8734 230
A5AA9 435
4J5AA 192
2Q222 540
65922 751
A9A7A 664
56598 906
99929 896
6Q67T 268
25J5J 198
J47AJ 45
262A6 63
4TKTK 571
622JT 480
K8KQK 574
47477 24
K2222 696
A9J28 342
Q4Q4Q 783
722T2 704
529A5 888
7TKT5 151
AA9AA 953
8KT8J 608
QQQJT 81
Q6K82 231
4TAAA 796
QK5J9 133
A83K2 634
6J666 662
75TJ8 77
T3444 148
QQ222 383
K4K77 91
T8TTT 178
A3K65 902
62225 413
TA5TT 19
996A6 826
6QQKK 946
888K8 183
56289 738
K8J28 887
55JKK 779
99K29 520
7623J 326
26222 918
39K86 853
T2Q63 975
27T2J 189
TTTJT 76
66994 686
78KKK 994
88333 482
A33A7 43
626TT 64
555J5 576
22T22 104
T44A4 439
2J666 79
A85Q6 978
3TTTT 59
78K47 865
4T6T4 602
27K55 546
T9K9T 831
66299 702
37444 706
J6825 309
T444T 426
J6QQQ 27
A2A8A 337
A97A7 499
29752 666
QTQKQ 191
KKK43 429
KKQKJ 583
AAA22 467
Q8898 813
J55JK 610
666KJ 18
Q3T6J 269
5T553 747
8T857 912
T6TA5 809
2J2J2 592
Q7QQQ 135
57K64 356
7374Q 113
48484 8
52992 790
AJ34Q 419
7777K 263
766J6 271
62967 745
ATA9A 899
K6K4K 895
3323J 52
AT423 965
JJJ8J 644
64444 500
A65KQ 188
75766 637
T2QT7 29
54527 661
5JJ5A 101
K3KJ3 453
33A33 991
52J52 910
2J48K 665
J8K26 163
J3JJ3 834
55K55 884
K63K2 376
3K2KK 721
892JK 205
57472 550
75JT3 11
Q5555 731
93738 112
AA466 312
66744 808
389KQ 924
K2JAJ 357
22233 71
JKKKJ 591
2Q294 762
QK58T 40
686J8 210
94499 493
QQ9QQ 421
K45KK 373
QKTA8 400
JJT56 521
QK9KT 812
67575 397
2KTQT 635
T4K3Q 1000
A88QQ 423
88J8J 573
56656 867
49454 518
T66T4 30
KJK7T 172
22422 961
34T34 530
T6TTT 5
K3KJK 857
6J583 817
K73JA 72
6K646 607
JK222 199
AAQA6 672
A7J68 488
9K5TQ 579
9K999 80
9JQQ9 715
39J34 533
6K978 633
2668A 150
2J322 657
KA2KA 934
28K82 850
TQK97 490
KKQJ9 361
4J453 937
2888T 314
6K666 193
T94A3 254
39KJJ 613
6336Q 350
45455 292
K7A89 741
45K4T 656
4Q63A 805
3T353 674
88778 305
277J2 889
33565 638
63333 799
54884 69
J9AAA 752
65AKT 28
357QT 128
74JA5 242
977J9 950
67676 838
369K4 246
9947A 306
Q6TJ9 97
53624 821
687K5 765
TA397 524
A6A66 218
AJ577 333
8J6KK 181
T77TA 647
3A8KQ 871
TQK42 955
QAAJA 220
48444 16
JTT7T 458
55A5J 845
33T33 232
83T8T 737
T4928 39
9AQ27 905
336JA 281
3Q4QQ 549
44699 432
72Q8Q 10
A7392 753
544K5 131
85458 806
33QAA 897
A55A5 279
2226J 485
T7388 207
5843T 849
4Q279 971
7J3Q7 652
T77QQ 643
86868 911
J3K77 718
2JQ94 954
A4449 963
T3KKK 742
8QATA 877
88336 6
7KJ8K 651
958QA 94
QJKQ4 987
96573 631
Q7586 719
Q66KA 130
A3TT2 247
AA385 894
QKQ2Q 629
QK44A 872
Q2T24 302
62QT8 616
7T5AT 225
44443 396
95299 156
43434 7
27T98 901
99J74 441
QA8K7 926
96967 35
3AAAA 789
68J26 484
83632 387
67666 158
T8TT4 615
495Q5 415
T9QA7 38
7JQT5 835
93996 449
97J99 998
2A2Q2 345
9ATTJ 655
37KQ7 495
A52J3 886
999AT 777
7K48K 932
T5T52 31
33392 99
43333 997
T6TT9 528
KAKK7 593
49837 473
4A73K 679
5628A 758
82537 245
99K9K 511
KKKK5 114
222A6 325
JJ553 379
2K777 89
777J7 641
T9TTT 262
K7K3K 681
222J2 129
J9JJ9 447
AQAAA 132
TJ77J 137
7844T 847
97QQQ 691
25J79 161
5Q22Q 154
Q4Q73 941
39773 468
9T92T 144
99297 75
4288K 392
TA3K5 107
8T8T8 223
7JTJ4 141
3K272 235
QQ5K7 773
TJA83 78
6Q7A2 142
696J9 227
QKKA6 298
222J8 701
JA93A 117
383AA 455
K3972 287
3433J 446
TTT2T 854
K9J99 184
8848K 224
KQ995 322
592Q2 307
9J676 95
7Q9J4 555
AA8Q7 102
9QQQ2 670
A6A2A 431
T844K 319
34534 802
9AQT5 409
7TJ62 875
TKQTK 406
74J77 219
KK55K 733
J8529 852
444Q2 478
98282 829
A9369 483
JJQQQ 430
5T2AJ 794
K2495 698
8A27J 50
33JJK 119
2K5K5 422
AJ957 47
76776 501
T593J 557
99974 951
KK2K2 203
6J777 259
QKJ33 368
88J99 407
94969 614
Q9J53 754
587K2 855
93Q28 504
7788J 682
7J7JQ 66
KKTTT 474
36343 70
4K3AJ 785
KKK6A 605
T9Q88 164
4Q445 139
7676K 67
89888 3
AATAJ 182
99977 374
644QJ 240
77577 770
QQ47Q 168
KAQKK 588
KKKK9 123
7K665 646
955J8 394
8A566 244
88JJJ 323
9K4T6 531
2922A 283
TAA22 673
J935K 505
Q52TK 358
J334K 475
2JK44 180
T64KA 335
4T6T9 553
226Q2 564
T674K 945
4A34T 457
QJQQQ 61
66686 575
AA9J6 554
KKKK4 171
QTQKK 700
99Q95 4
5TQ5Q 921
645TK 177
95Q5Q 155
J347T 728
2927K 748
3A6J6 461
9TAK7 567
48T69 343
QQQ77 321
T42TT 522
5Q656 514
KK8KK 363
K3293 784
2QKK6 249
4AA4A 534
23233 173
Q3933 12
33A87 880
KK3A7 725
6J326 347
3K3KK 98
AQAA5 346
26262 959
J493J 294
J32J6 763
63J4K 866
3T366 201
Q43KQ 297
66KK6 939
J6KJT 677
JAJ6T 517
QJ899 487
QKKQ5 450
5T2JT 121
3T982 695
999JT 772
55444 265
J9A99 800
337JJ 211
4A683 722
2KKQJ 697
22225 922
K8KK8 964
53Q93 366
47AJ6 174
A28Q8 603
99222 874
23QK2 539
A256K 26
66688 943
36676 626
JK952 145
94A42 516
KJKKK 720
K55K6 42
Q44Q4 22
"#;
