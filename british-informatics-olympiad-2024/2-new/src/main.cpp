#include <functional>
#include <iostream>
#include <math.h>
#include <string>

using namespace std;

// a custom type, a SetFunc expects an index and returns an int
using SetFunc = function<int(int i)>;

// 2, 4, 6, 8, 10, ...
int E(int i) { return 2 * i; }

// 1, 3, 5, 7, 9, ...
int O(int i) { return 2 * i - 1; }

// 1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, ...
int T(int i) {
  if (i <= 1) {
    return 1;
  }

  // math stuff with triangular numbers
  for (int k = i - 1; k > 0; k--) {
    if (floor(sqrt(8 * k + 1)) == ceil(sqrt(8 * k + 1))) {
      return (-1 + sqrt(1 + 8 * k)) / 2 + 1;
    }
  }

  // fallthrough case
  return 0;
}

// combines two sets into a combined set
// this might be slightly weird, it's a function that takes two functions as
// parameters and returns another function! Everything is a function as if we're
// in haskell! But C++ is awesome and can do these things with ease.
SetFunc C(SetFunc left, SetFunc right) {
  if (!left) { // Check if left is empty
    return [=](int i) -> int { return right(i); };
  }
  return [=](int i) -> int { return right(left(right(i))); };
}

// get string length
// it's ridiculous that I had to implement this myself but nothing is perfect
// and C++ is no exception
int str_len(string inp) {
  int i;
  for (i = 0; inp[i]; i++)
    ;
  return i;
}

// find the index of the closing bracket related to the one at open_pos
int closing_bracket_pos(string text, int open_pos) {
  int counter = 1; // to keep track of the pairs of brackets nested inside

  // from the character after the opening bracket until the end of the string
  for (int searching_at = open_pos + 1; searching_at < str_len(text);
       searching_at++) {
    // it's like an equation, +1 for ( and -1 for )
    if (text[searching_at] == '(')
      counter++;
    else if (text[searching_at] == ')')
      counter--;

    // stop once you have passed the same number of "("s as ")"s
    if (counter == 0)
      return searching_at;
  }

  return -1;
}

// create the full combined set as described by the text
// this function recursively calls itself on the contents of brackets
// otherwise it iterates and combines E,O, and T sets normally (left and right)
SetFunc get_combined_set(string text) {
  SetFunc result_set; // the result set

  int i = 0; // string iteration
  while (i < str_len(text)) {
    switch (text[i]) {
    // Combine what we already have (left) and new content (right)
    case 'E':
      result_set = C(result_set, E);
      i++;
      break;
    case 'O':
      result_set = C(result_set, O);
      i++;
      break;
    case 'T':
      result_set = C(result_set, T);
      i++;
      break;

      // notice the recursive bit
      // we have dealt with the content as much as we could have, and now we
      // leave the rest to our "child"
    case '(':
      // find the position of the closing bracket
      int end_bracket_pos = closing_bracket_pos(text, i);

      // recursively handle the bracket substring
      SetFunc bracket_result =
          get_combined_set(text.substr(i + 1, end_bracket_pos - i - 1));
      result_set = C(result_set, bracket_result);

      // skip to the character after the closing bracket as we've dealt with
      // the bracket content recursively
      i = end_bracket_pos + 1;
      break;
    }
  }

  return result_set;
}

int main() {
  // example use of the combine function
  SetFunc combined_set = C(E, C(O, E));
  cout << combined_set(3) << " same as using " << C(E, C(O, E))(3) << endl;

  string input_str;
  int input_num;
  // c input from user
  cin >> input_str >> input_num;

  combined_set = get_combined_set(input_str);
  cout << combined_set(input_num) << endl;
}
