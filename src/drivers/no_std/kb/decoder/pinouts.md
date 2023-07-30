# pinoutsl

```
        XO      X1      X2      X3  |   X4      X5      X6      X7
------------------------------------+--------------------------------
YO      ESC     TAB     A       Z   |   /       )       *       ESC
                                    |
Y1      1!      Q       D       X   |   DOWN    UP      LEFT    RIGHT
                                    |
Y2      20      W       S       C   |   0       4       8       (
                                    |
Y3      34      E       H       V   |   1       5       9       -
                                    |
Y4      4$      R       F       S   |   2       6       .       RETURN
                                    |
Y5      6"      Y       G       N   |   3       7       +       ,
                                    +----------------------------------
Y6      5%      T       J       M       \|      `~      RETURN  DELETE

Y7      7&      U       K       ,<      +=       P      UP       DOWN

Y8      8*      I       ;:      .>      0)       [{     SPACE   LEFT

Y9      9(      O       L       /?      -_       ]}      '"      RIGHT
```

# todo

| IIe | Pico | Signal  |
| --- | ---- | ------- |
| 5   | GP5  | SW1     |
| 7   | GP7  | SW0     |
| 11  | GP11 | Control |
| 15  | GP9  | RESET   |
| 24  | GP26 | Shift   |

## IIe <-> Pico

| IIe | Pico | Signal   |
| --- | ---- | -------- |
| 1   | GP2  | Y0       |
| 2   | GP3  | Y1       |
| 3   | +5V  | +5V      |
| 4   | GP4  | Y2       |
| 5   | GP5  | SW1      |
| 6   | GP14 | Y3       |
| 7   | GP7  | SW0      |
| 8   | GP8  | Y4       |
| 9   | GND  | CAPSLOCK |
| 10  | GP10 | Y5       |
| 11  | GP11 | Control  |
| 12  | GP12 | Y8       |
| 13  | GND  | GND      |
| 14  | GP13 | X0       |
| 15  | GP9  | RESET    |
| 16  | GP15 | X2       |
| 17  | GP16 | X7       |
| 18  | GP17 | X1       |
| 19  | GP18 | X5       |
| 20  | GP19 | X3       |
| 21  | GP20 | X4       |
| 22  | GP21 | Y9       |
| 23  | GP22 | Y6       |
| 24  | GP26 | Shift    |
| 25  | GP27 | Y7       |
| 26  | GP28 | X6       |

## IIe (X Column) <-> Pico

| IIe | Pico | Signal |
| --- | ---- | ------ |
| 14  | GP13 | X0     |
| 18  | GP17 | X1     |
| 16  | GP15 | X2     |
| 20  | GP19 | X3     |
| 21  | GP20 | X4     |
| 19  | GP18 | X5     |
| 26  | GP28 | X6     |
| 17  | GP16 | X7     |

## IIe (Y Rows) <-> Pico

| IIe | Pico | Signal |
| --- | ---- | ------ |
| 1   | GP2  | Y0     |
| 2   | GP3  | Y1     |
| 4   | GP4  | Y2     |
| 6   | GP14 | Y3     |
| 8   | GP8  | Y4     |
| 10  | GP10 | Y5     |
| 12  | GP12 | Y8     |
| 22  | GP21 | Y9     |
| 23  | GP22 | Y6     |
| 25  | GP27 | Y7     |

## IIe (Keyboard Connector) <-> Pico

the orientation of the notch is to the right.

### Left (Unnotched {even} - Top -> Down)

| IIe | Pico | Signal | color  |
| --- | ---- | ------ | ------ |
| 2   | GP3  | Y1     |        |
| 4   | GP4  | Y2     |        |
| 6   | GP14 | Y3     |        |
| 8   | GP8  | Y4     |        |
| 10  | GP10 | Y5     |        |
| 12  | GP12 | Y8     |        |
| 14  | GP13 | X0     |        |
| 16  | GP15 | X2     |        |
| 18  | GP17 | X1     | orange |
| 20  | GP19 | X3     | yellow |
| 22  | GP21 | Y9     | green  |
| 24  | GP26 | Shift  | blue   |
| 26  | GP28 | X6     | orange |

### Right (Notched {odd} - Top -> Down)

| IIe | Pico | Signal   | color  |
| --- | ---- | -------- | ------ |
| 1   | GP2  | Y0       |        |
| 3   | +5V  | +5V      |        |
| 5   | GP5  | SW1      |        |
| 7   | GP7  | SW0      |        |
| 9   | GP9  | CAPSLOCK |        |
| 11  | GP11 | Control  |        |
| 13  | GND  | GND      |        |
| 15  | GND  | RESET    |        |
| 17  | GP16 | X7       | blue   |
| 19  | GP18 | X5       | purple |
| 21  | GP20 | X4       | grey   |
| 23  | GP22 | Y6       | white  |
| 25  | GP27 | Y7       | black  |
