## Rush
> you do not need to rush, you just need Rush

Rush 可以帮助你预约交我办，但不保证一定能够预约上，现支持
- [超市](https://dailyreport.sjtu.edu.cn/market/)
- [理发](https://dailyreport.sjtu.edu.cn/haircut)

```
rush --help
```
可执行文件见 [Release](https://github.com/junhaideng/rush/releases)

### 超市
```
USAGE:
    rush market [OPTIONS] <COOKIE>

ARGS:
    <COOKIE>    

OPTIONS:
    -a, --asc <ASC>            [default: true]
        --always <ALWAYS>      [default: true]
    -c, --choices <CHOICES>    [default: 1,2,3]
    -h, --help                 Print help information
```

`choice` 的对应关系为
```
1 => 西区教超 
2 => 东区教超
3 => 罗森 
```
`asc` 为 `true` 表示预约越早的，否则优先预约越晚的

`always` 表示不断循环进行预约，设置为 `false` 表示仅预约一次

### 理发
```
USAGE:
    rush haircut [OPTIONS] <COOKIE>

ARGS:
    <COOKIE>    

OPTIONS:
    -a, --asc <ASC>            [default: true]
        --always <ALWAYS>      [default: true]
    -c, --choices <CHOICES>    [default: 2,3,4]
    -h, --help                 Print help information
```
`choices` 的对应关系为
```
2 => 二餐理发
3 => 三餐理发
4 => 四餐理发
```

### 说明

仅供学习使用，切勿用于违规交易，**请把机会留给需要的人** :star:
