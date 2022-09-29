public class Algorithms
{
    public int StrStr(string text, string pattern)
    {
        int m = text.Length;
        int n = pattern.Length;
        if(n == 0) {
            return 0;
        }
        var next = new int[n];
        for(int i = 1, j = 0; i < n; i++) {
            while(j > 0 && pattern[i] != pattern[j]) {
                j = next[j - 1];
            }
            if(pattern[i] == pattern[j]) {
                j++;
            }
            next[i] = j;
        }

        for(int i = 0, j = 0; i < m; i++) {
            while(j > 0 && text[i] != pattern[j]) {
                j = next[j - 1];
            }
            if(text[i] == pattern[j]) {
                j++;
            }
            if(j == n) {
                return i - n + 1;
            }
        }

        return -1;
    }
}