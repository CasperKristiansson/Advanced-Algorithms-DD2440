import subprocess

data = '''10
95.0129 61.5432
23.1139 79.1937
60.6843 92.1813
48.5982 73.8207
89.1299 17.6266
76.2097 40.5706
45.6468 93.5470
1.8504 91.6904
82.1407 41.0270
44.4703 89.3650
'''

data = '''500
75.1535 26.6743
21.9653 33.0668
70.1337 53.8589
40.2761 97.1073
64.6744 20.6556
28.516 39.1959
66.0919 39.0741
36.8484 69.0656
78.4573 42.6025
47.3932 81.3664
92.6246 20.0589
36.0751 58.7355
42.7786 25.7187
42.9575 45.7561
64.6358 27.074
43.4764 86.5102
10.7176 89.1535
23.2574 53.086
49.4925 27.8893
95.0212 28.5405
56.4737 95.4859
26.9958 32.8175
88.6797 19.1819
46.6432 96.0303
75.432 37.8638
36.9794 67.4905
68.6731 10.1223
73.3075 33.6169
8.3918 6.074
86.7793 9.5607
46.803 69.5665
78.8993 6.8035
52.9036 87.6146
52.847 75.0565
10.2885 47.6633
44.4773 18.1164
26.2114 36.9654
11.419 4.4044
49.4097 39.0444
30.6549 98.382
41.4168 23.2909
17.4708 8.3324
73.2533 23.3455
78.8933 38.1878
29.2528 75.7738
39.6208 25.9927
21.3092 86.9588
15.791 57.4018
8.8709 27.0064
45.436 9.4073
39.5464 84.6268
65.3299 76.932
50.7245 81.2195
84.8514 45.489
34.7046 65.0087
2.4306 90.4349
27.7387 39.7238
97.2717 28.0785
71.0375 53.5761
64.9397 85.1567
17.6445 85.5585
18.3701 61.1825
34.3464 14.6325
29.6768 6.1733
5.6734 60.8775
95.4167 36.6864
87.3751 38.4806
31.6648 74.5655
26.9029 64.4661
15.1211 28.7572
90.0144 15.5957
68.3026 30.5008
39.4109 10.1588
67.7791 87.2563
4.2263 88.9715
75.3134 65.2559
66.8246 22.8859
95.678 67.857
37.8415 1.904
70.5067 27.8791
15.1321 78.5292
38.8968 73.0988
5.9609 48.604
76.2616 50.0159
36.8127 55.4
83.2352 42.4932
81.0879 17.5409
68.5241 65.1974
14.2658 8.6999
21.9923 22.1399
46.6352 42.3651
15.3813 8.305
61.9647 21.46
59.7787 27.1661
43.4061 7.0157
4.197 98.1583
34.3467 19.1835
11.0546 35.0946
26.4049 70.8698
81.8517 74.6467
49.0345 15.3229
12.1671 5.1028
3.921 6.4425
52.7448 95.9365
85.8356 20.877
10.0388 64.0992
14.8763 86.0118
58.392 89.5212
94.742 48.5452
50.4746 71.1535
81.6504 57.5045
7.7007 27.1435
79.0085 6.0233
37.5652 25.8274
41.1845 97.6467
36.1535 32.2695
98.9891 27.3653
59.2049 79.8224
54.6948 47.2938
38.1818 34.6953
29.919 98.1484
16.6831 76.7605
91.5283 84.1002
49.3601 20.4513
94.3546 46.4844
17.1002 66.1926
11.0817 78.0008
19.6216 78.5434
43.1565 45.3977
91.0628 69.4933
69.5801 56.6457
34.7344 53.3145
53.3013 33.2683
93.8066 21.1967
95.6796 19.454
65.639 14.0478
49.9973 76.5627
47.8209 15.1056
41.614 14.2787
98.2513 77.4149
62.5637 92.8725
52.1072 62.0464
14.3405 83.3887
40.4306 31.6839
3.3974 8.9434
33.8526 77.8605
50.7783 96.9229
59.9981 95.9586
7.2334 61.1647
96.9601 49.7023
57.2741 82.6633
77.2054 15.8875
9.0761 4.9511
38.2749 64.6544
7.3004 42.518
22.547 57.7765
46.6988 5.5335
96.3237 8.8022
45.3261 69.2228
91.9075 67.8416
36.787 23.6863
30.6648 89.5758
87.0899 85.1206
45.9901 26.8774
91.6001 40.4692
33.4869 75.1002
8.7635 74.5933
2.4979 15.2398
73.5436 56.0933
69.9896 77.8967
50.9128 92.0437
10.5783 31.214
1.7685 4.9711
77.3759 70.8793
59.5357 39.6033
75.1086 7.6983
98.2173 44.2306
80.0167 50.7468
66.6013 82.398
49.3593 64.6952
57.4218 12.0742
9.006 22.4371
59.6478 36.5479
89.3034 94.4752
29.8252 85.4969
73.2142 77.3724
76.1862 50.3743
6.8144 88.999
33.963 93.6336
88.5986 72.2326
60.1149 79.2333
75.1969 61.9864
1.0495 98.3587
72.0765 98.609
75.8793 95.8012
81.5367 67.4002
26.0131 75.7949
64.3386 79.6333
78.2269 4.1349
13.8046 97.7576
10.527 54.027
28.2897 77.2368
44.8215 39.6383
76.744 14.4369
18.3575 95.803
38.0427 32.4733
79.7997 85.4807
9.0429 40.9472
64.2535 94.5527
63.581 72.0118
12.0923 39.4055
63.411 39.6155
47.8631 85.5188
8.4802 88.1129
85.4199 90.4184
27.5535 6.1296
80.336 51.4933
76.9127 63.5816
41.3041 30.2999
68.9364 36.8974
92.1573 72.8779
1.3337 20.2197
56.3093 12.5013
60.775 8.2016
72.4187 77.3339
8.2633 21.843
55.8635 92.2323
84.0448 60.342
52.0191 25.8442
74.1675 41.7747
7.2922 89.9871
66.7202 24.0323
46.2462 38.3438
33.4944 48.655
46.7296 79.8981
13.0946 57.658
79.6332 98.6093
35.1451 8.1511
56.1372 86.5382
25.3924 98.4553
4.5342 23.2856
24.0368 33.7806
3.4427 62.0568
6.6074 64.367
82.3326 35.9126
53.6509 31.6079
74.4609 61.295
81.2374 92.1881
71.1631 47.2912
44.5936 56.5139
6.8653 97.149
50.5196 96.9014
37.4447 65.9452
66.6828 80.7339
59.8765 10.7406
56.6133 61.4433
69.9814 66.2893
58.2103 81.9012
23.653 45.829
74.8679 87.6925
62.435 30.3474
35.3163 16.8899
10.7585 35.1182
10.8238 42.0461
34.7328 1.9236
54.7287 70.6702
62.9082 37.8901
25.9403 91.7099
62.2447 39.8792
71.0163 38.1345
5.8231 7.1284
40.7849 7.536
4.7699 8.5772
53.4435 98.7847
96.9429 11.979
24.2699 74.7756
52.4406 14.1017
36.9312 69.1718
37.8076 63.5435
23.1212 50.5732
59.9654 45.3029
13.2209 8.2055
52.8985 76.0075
76.3857 37.3165
70.6622 22.3053
72.1371 46.6083
63.1685 37.7069
52.1462 10.0854
86.769 73.0499
2.2488 6.8205
98.9593 37.1796
16.9791 58.0277
1.4948 87.1316
8.3107 28.2865
52.4193 67.8031
11.608 63.475
53.0309 93.4651
81.9609 84.5942
26.4506 23.5682
24.6506 1.3968
95.0969 77.0491
59.5722 30.7773
88.4733 22.6147
60.1887 61.4713
28.8782 1.8932
71.8475 99.3538
79.2699 58.9903
69.0273 71.7554
3.9586 85.9462
13.4221 95.8689
1.5741 33.7938
31.6414 25.7263
80.0703 48.801
40.845 23.0787
6.6841 90.0657
34.7205 56.6291
53.2384 70.1535
82.8513 73.5075
27.9061 53.4281
22.6433 63.099
81.3026 33.4702
9.8702 24.5223
80.4851 62.2036
53.7793 42.0716
44.6467 83.5472
21.71 66.9808
89.3535 12.0063
27.6521 11.2122
59.8138 59.4328
43.7337 43.5638
80.912 50.2168
37.7421 94.7485
38.3386 83.3183
92.7444 41.4229
73.8642 45.8231
58.7706 91.3044
12.2469 58.206
6.5993 62.6695
15.7609 23.4991
62.4701 69.9017
31.8119 53.6013
55.9073 66.0698
68.3209 61.5669
45.0927 42.3654
14.6705 30.7562
14.5961 71.0016
36.7667 92.9475
75.5721 86.3952
62.2676 11.6466
99.0164 16.9706
99.5493 56.1628
99.7515 84.4824
38.254 79.7125
38.3144 13.2755
59.0788 13.9568
77.3758 36.9186
77.2133 99.7914
12.1321 77.1081
85.8085 25.9039
73.0019 21.6963
12.0262 30.8971
69.8167 8.0033
24.017 44.4697
49.8239 38.6447
4.5318 47.6257
18.8704 2.1698
70.3823 60.9669
57.8687 12.736
21.2886 51.1904
4.9724 7.325
60.1611 41.1935
57.9367 39.564
13.3756 56.2406
1.4492 30.6631
91.4212 71.1556
23.2842 25.8041
52.3509 54.4143
29.8725 99.3371
61.9717 24.8499
60.3666 17.5855
17.7142 80.6564
47.4353 46.1442
74.8661 61.5711
29.216 56.4509
12.0582 57.4409
73.6505 24.2404
78.2056 67.9692
27.5781 27.7082
7.0754 62.0973
29.162 55.9551
64.5317 89.4321
69.9497 6.4483
50.1617 27.5732
49.7102 11.4249
53.5299 16.6399
87.7133 21.4747
71.554 55.898
98.5978 91.1811
24.7335 14.4932
49.1496 20.989
4.1446 32.1213
46.6446 12.3388
50.611 35.9786
58.7454 29.052
92.844 97.8693
9.2025 14.1378
72.6639 57.1809
21.7834 95.0576
89.0999 22.0201
75.948 22.6275
90.4659 82.1617
62.3302 6.1463
53.2548 21.5851
95.5126 76.9768
81.098 7.7009
55.0162 55.7431
13.7585 54.7264
15.0 2.1873
39.4431 45.9827
41.2358 9.8042
44.4174 36.0257
46.8331 3.0373
73.6123 61.776
87.7134 16.1236
76.6594 10.5064
60.1154 9.6782
94.0964 5.5281
12.0653 25.1263
48.6526 60.3268
81.1312 26.6133
6.1122 92.0913
63.7625 46.9053
14.7778 31.4866
84.5977 86.0288
3.2051 93.3844
67.2901 3.8098
89.8504 12.1192
43.4979 92.5148
81.1002 63.3983
72.8238 99.4657
96.0763 17.604
57.5794 22.6601
3.9344 83.9291
5.3965 14.9215
14.5793 43.4342
44.9354 81.4012
88.4145 94.4818
76.2705 69.2728
33.1509 13.9898
50.8018 60.5711
30.0146 45.5981
18.0516 42.6049
89.7605 68.0263
45.3879 74.4616
70.109 36.9703
21.4921 44.2448
89.0798 87.7261
20.2465 14.9613
42.3991 32.3
15.5382 23.5624
20.0419 21.6946
56.6533 51.8299
50.3263 76.5141
44.4181 38.7211
51.5877 18.9074
29.0662 39.2271
91.8689 86.5909
65.36 42.9248
46.1577 85.739
85.9033 11.045
34.7833 73.9818
77.4117 75.2853
59.4889 81.4662
63.6816 55.5274
50.019 92.0652
75.8259 60.0481
10.2964 7.7462
41.1344 23.9752
69.7849 59.8207
43.0615 45.6915
79.6373 85.3755
90.6921 4.6936
41.2745 2.3505
98.6062 67.9922
73.5046 82.7278
26.4452 52.3608
20.2679 46.1456
33.596 78.8485
21.0464 5.1363
80.3241 79.3248
60.2032 84.7171
84.6258 24.8851
70.8978 44.5933
7.4321 84.7064
54.4061 68.4386
41.7452 56.9654
68.7288 39.6597
80.2483 8.0318
7.9366 11.7187
71.8662 8.2111
'''

command = ['cargo', 'run', '--']

process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

stdout, stderr = process.communicate(input=data)

print("STDOUT:", stdout, sep='\n')
print("STDERR:", stderr)
