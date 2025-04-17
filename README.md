### Simplest possible program for int base conversions


Don't do 
```bash
python -c 'print("{0:#032b}".format(0xfcd1a11))'
```
do
```bash
x 0xfcd1a11.b.32
```
Examples
```bash
x 0xc0de.b.16 
0b1100000011011110

echo "0b0101
0xf0c
0o777" | x .dec

5
3852
511
```
