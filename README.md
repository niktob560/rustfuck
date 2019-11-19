# rustfuck [WIP]
Basic brainfuck interpreter written in rust  

Interpretate from file:
```bash
rustfuck -o /path/to/file
```
  
Interpretate from stdin (^D required):
```bash
rustfuck -i
```
  
Input (, instruction):
```bash
rustfuck -o examples/addone.bf
>>>1
2
```
rustfuck will print '>>>' to get a char from stdin
  
Show help:
```bash
rustfuck -h
```
