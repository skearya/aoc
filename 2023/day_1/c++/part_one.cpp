#include <iostream>
#include <vector>
#include <fstream>

const char numbers[] = {'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'};

int main()
{
    std::ifstream input("../input.txt");
    std::vector<int> current_line;
    int sum = 0;

    char ch;
    while (input.get(ch))
    {
        if (ch == '\n')
        {
            sum += 10 * current_line[0] + current_line[current_line.size() - 1];
            current_line.clear();
            continue;
        }

        for (char number : numbers)
        {
            if (ch == number)
            {
                current_line.push_back(ch - '0');
                break;
            }
        }
    }

    std::cout << sum << std::endl;

    input.close();
    return 0;
}
