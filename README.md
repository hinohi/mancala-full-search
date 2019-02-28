# Mancala

## ゲームルール

TBA

## 結果

### スコア

| 穴の数\石の数 |  1 |  2 |  3 |  4 |  5 |   6 |
| ----------: | -: | -: | -: | -: | -: | --: |
|           1 |  1 |  1 |  5 |  3 | -2 |  -2 |
|           2 |  1 |  2 | -1 | -5 | 10 |   3 |
|           3 |  2 |  4 |  2 |  3 |  9 | -10 |
|           4 |  1 |  5 |  5 |  3 |  2 |   0 |
|           5 |  1 |  4 |  7 |    |    |     |
|           6 |  1 |  3 |    |    |    |     |
|           7 |  1 |    |    |    |    |     |
|           8 |  1 |    |    |    |    |     |

### 盤面数

| 穴の数\石の数   |        1 |         2 |        3 |       4 |        5 |         6 |
| ------------: | -------: | --------: | -------: | ------: | -------: | --------: |
|             1 |        2 |         2 |        2 |       2 |        3 |         5 |
|             2 |        8 |        23 |       88 |     110 |      102 |       671 |
|             3 |       64 |      1239 |     6928 |   34046 |   104844 |    207547 |
|             4 |     1128 |     63879 |   797036 | 5092663 | 27174327 | 100509811 |
|             5 |    20170 |   3325441 | 83874363 |         |          |           |
|             6 |   346620 | 167533563 |          |         |          |           |
|             7 |  5908812 |           |          |         |          |           |
|             8 | 97853783 |           |          |         |          |           |

### log10(盤面数)

| 穴の数\石の数   |     1 |     2 |     3 |     4 |     5 |     6 |
| ------------: | ----: | ----: | ----: | ----: | ----: | ----: |
|             1 | 0.301 | 0.301 | 0.301 | 0.301 | 0.477 | 0.699 |
|             2 | 0.903 | 1.362 | 1.944 | 2.041 | 2.009 | 2.827 |
|             3 | 1.806 | 3.093 | 3.841 | 4.532 | 5.021 | 5.317 |
|             4 | 3.052 | 4.805 | 5.901 | 6.707 | 7.434 | 8.002 |
|             5 | 4.305 | 6.522 | 7.924 |       |       |       |
|             6 | 5.540 | 8.224 |       |       |       |       |
|             7 | 6.772 |       |       |       |       |       |
|             8 | 7.991 |       |       |       |       |       |

### 盤面の順列組み合わせからの予想

穴の数を$p$石の数を$s$とすると、穴は全部で$2p$個石は全部で$2ps$個ある。
単純に$2ps$個の区別できない石を$2ps$個の穴に入れる問題だと思うと、
その組み合わせの数は$2ps+2p$個の区別できない棒と$2ps$個の区別できない石を一列に並べる組み合わせの数に一致する。
よって$p,s$が与えられたときの理論的な盤面数$B(p,s)$は
$$
B(p,s) = \frac{(2ps + 2p)!}{(2ps)!(2p)!}
$$
となる。

有効な盤面が理論的に存在しうる盤面数にしめる割合

| 穴の数\石の数   |         1 |         2 |         3 |         4 |         5 |         6 |
| ------------: | --------: | --------: | --------: | --------: | --------: | --------: |
|             1 | 3.333e-01 | 1.333e-01 | 7.143e-02 | 4.444e-02 | 4.545e-02 | 5.495e-02 |
|             2 | 1.143e-01 | 4.646e-02 | 4.835e-02 | 2.270e-02 | 9.599e-03 | 3.277e-02 |
|             3 | 6.926e-02 | 6.674e-02 | 5.147e-02 | 5.734e-02 | 5.383e-02 | 3.956e-02 |
|             4 | 8.765e-02 | 8.685e-02 | 7.578e-02 | 6.622e-02 | 7.201e-02 | 7.076e-02 |
|             5 | 1.092e-01 | 1.107e-01 | 9.895e-02 |           |           |           |
|             6 | 1.282e-01 | 1.338e-01 |           |           |           |           |
|             7 | 1.473e-01 |           |           |           |           |           |
|             8 | 1.628e-01 |           |           |           |           |           |
