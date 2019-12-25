# longest_substring_without_repeating_characters

Given a string, find the length of the **longest substring** without repeating characters.

**Examples:**

Given `"abcabcbb"`, the answer is `"abc"`, which the length is 3.

Given `"bbbbb"`, the answer is `"b"`, with the length of 1.

Given `"pwwkew"`, the answer is `"wke"`, with the length of 3. Note that the answer must be a **substring**, `"pwke"` is a *subsequence* and not a substring.



找s中最长的连续子串。



思路：两个指针i和j，i指向最长子串的右边界，j指向左边界，在i向右移动的过程中，记录每个字符串的位置，如果字符已经扫描过了且在左右边界内，就将左边界移动到相同的那个元素的后面，这样保持左右边界内不存在重复的字符。每次移动过程中都计算下左右边界内的字符数，并且和最大值进行比较。



