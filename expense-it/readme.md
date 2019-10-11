# Expense it!

You've just returned from a sucessful startup seeding round in Tokyo. You had a
lot of fun and spent a lot of company money. Now you need to submit your
receipts for reimbursement (Don't worry, you should haave raised plenty of money
to cover it). Thankfully, you have all of your expenses neatly organized in a
text document. Naturally, the expense descriptions are in Japanese
Unfortunately, that description is now backwards and all of the Hiragana
characters have become Katakana characters and vice versa (It must of been the
shift in the time zone)... On top of that, the finance department requires all
small half width kana to be replaced with standard half width kana because they
are to hard to read. Fix your expense report quickly so you can afford to go on
your next trip!

## Description

An expense report consist of an unknown number $n$ entries begining with '$'.
The '$' character will only appear as the first character of an entry. The
following lines will contain a descripton of $x$ characters between 0 and 2^16
characters with 40 characters on each line. The only Hiragana, Katakana, and
small half width Kana that will be used are in the character list below.

```
Hiragana characters:
'あ' | 'い' | 'う' | 'え' | 'お' |
'か' | 'き' | 'く' | 'け' | 'こ' |
'が' | 'ぎ' | 'ぐ' | 'げ' | 'ご' |
'さ' | 'し' | 'す' | 'せ' | 'そ' |
'ざ' | 'じ' | 'ず' | 'ぜ' | 'ぞ' |
'た' | 'ち' | 'つ' | 'て' | 'と' |
'だ' | 'ぢ' | 'づ' | 'で' | 'ど' |
'な' | 'に' | 'ぬ' | 'ね' | 'の' |
'は' | 'ひ' | 'ふ' | 'へ' | 'ほ' |
'ば' | 'び' | 'ぶ' | 'べ' | 'ぼ' |
'ぱ' | 'ぴ' | 'ぷ' | 'ぺ' | 'ぽ' |
'ま' | 'み' | 'む' | 'め' | 'も' |
'ら' | 'り' | 'る' | 'れ' | 'ろ' |
'や' | 'ゆ' | 'よ' | 'わ' | 'ん'

Katakana characters:
'ア' | 'イ' | 'ウ' | 'エ' | 'オ' |
'カ' | 'キ' | 'ク' | 'ケ' | 'コ' |
'ガ' | 'ギ' | 'グ' | 'ゲ' | 'ゴ' |
'サ' | 'シ' | 'ス' | 'セ' | 'ソ' |
'ザ' | 'ジ' | 'ズ' | 'ゼ' | 'ゾ' |
'タ' | 'チ' | 'ツ' | 'テ' | 'ト' |
'ダ' | 'ヂ' | 'ヅ' | 'デ' | 'ド' |
'ナ' | 'ニ' | 'ヌ' | 'ネ' | 'ノ' |
'ハ' | 'ヒ' | 'フ' | 'ヘ' | 'ホ' |
'バ' | 'ビ' | 'ブ' | 'ベ' | 'ボ' |
'パ' | 'ピ' | 'プ' | 'ペ' | 'ポ' |
'マ' | 'ミ' | 'ム' | 'メ' | 'モ' |
'ラ' | 'リ' | 'ル' | 'レ' | 'ロ' |
'ヤ' | 'ユ' | 'ヨ' | 'ワ' | 'ン'

Small Half Width Kana:
'ｧ' | 'ｨ' | 'ｩ' | 'ｪ' | 'ｫ'

Standard Half Width Kana:
'ｱ' | 'ｲ' | 'ｳ' | 'ｴ' | 'ｵ'
```

## Example
### Input:
```
$600.00 Rental Car
ーかたんれ
$900.00 New business suit
ロビセイシラタア
$20.00 Conveyor belt sushi
司寿転回
$1200.00 First class flight to Tokyo
といらふノｽﾗｸﾄｽｰｧﾌノヘ京東
$930.00 Hot Springs
。スマイテレワイトルアガ史歴ノ年０００，３デ泉温イ古番一デ本日ハ泉温後道ルアニ市
山松県媛愛デ中ノ泉温ルアンサクタ。スマリアガ泉温ニ中本日，デノイ多ガ山火ハ本日。
ンセマキデハトコルレスワモ泉温，バエ言ト所名ノ本日
```

### Output:
```
$600.00 Rental Car
レンタカー
$900.00 New business suit
あたらしいせびろ
$20.00 Conveyor belt sushi
回転寿司
$1200.00 First class fight to Tokyo
東京へのﾌｱｰｽﾄｸﾗｽのフライト
$930.00 Hot Springs
日本の名所と言えば，温泉もわすれることはできません。日本は火山が多いので，日本中
に温泉があります。たくさんある温泉の中で愛媛県松山市にある道後温泉は日本で一番古
い温泉で３，０００年の歴史があるといわれています。
```


