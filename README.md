# Advent of Code 2024

- https://adventofcode.com/
- https://github.com/livexia/advent-of-code-2024

## 移除所有的 input.txt

根据社区和作者的规定，其实参与者并不允许上传 input 到公共仓库，我之前一直没有意识到这个问题，所以为了可确定性所以一直上传输入，今天意识到这个问题，所以删除了所有的输入，也许还有少量残留（不太可能）。

使用的方法来自社区：
1. [Puzzle Inputs](https://old.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs)
2. > git filter-repo --path-glob "2022/*/input" --invert-paths --force was the nice way to fix this for me. Edit the glob to match how you had inputs saved
    - 需要重新配置仓库远端地址，同时需要强制推送代码，以及 github 可能存在短暂的更新延时，以及可能的云端缓存，不是很确定，以及会丢失本地的 input 文件，也算是我的教训吧，下次记得要检查类似的问题。
    - [fragger](https://old.reddit.com/r/adventofcode/comments/18ehed6/re_not_sharing_inputs_psa_deleting_and_committing/kcroxma/)
3. [newren/git-filter-repo](https://github.com/newren/git-filter-repo)

## Day 1

太久没编程，还好大部分的记忆还在，有一点点手生，但是没什么问题。

## Day 2

第二天的题目并不困难，第一部分很简单，前后比较即可，第二部分虽然想有更好的方法，但是思路并不那么清晰，最后就暴力解决了，因为输入的问题，所以效率也不差。输入的数据每行的长度也不大，同时数据每行大都是有序的，而且也只考虑移除一个元素后数列的变化情况，所以实际上问题的求解空间就不大。如果输入数列趋于无序，同时需要计算删除最少元素以保证数列排序要求，那么求解空间就变得很大了，暴力实现也就不怎么现实了。在社区上简单浏览了一下也没有看见贴别好的方法，也许还要再等等吧，就这样了。

## Day 3

第三天的题目依旧不困难，但是在输入的处理上稍有麻烦，太久没编程都忘了还能用 regex 处理输入了，刚开始想要自己实现输入的匹配，但是没能下定决定，然后浪费了不少时间。虽然对 regex 并不是很熟练，但是基础还是有的，所以写个简单的表达式并没有问题，利用[网站](https://regex101.com/)简单的测试，再根据 [regex](https://docs.rs/regex/latest/regex/index.html) 的文档很容易的就实现了输入的解析和处理。对输入的计算其实很容易，再处理完输入之后很容易就可以得出结果。
