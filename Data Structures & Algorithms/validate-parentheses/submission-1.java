class Solution {
    public static boolean isValid(String s) {
        if (s.length() %2 != 0) { return false; }

        Deque<Character> res = new ArrayDeque<>();

        for (char c : s.toCharArray()) {
            if ( c == '{' || c == '(' || c == '[') {
                res.push(c);
            } else {
                if (res.isEmpty()) { return false; }
                else if ( c == '}' && res.peek() != '{') { return false; }
                else if ( c == ')' && res.peek() != '(') { return false; }
                else if ( c == ']' && res.peek() != '[') { return false; }
                res.pop();
            }
        }
        return res.isEmpty(); 
    }
}
