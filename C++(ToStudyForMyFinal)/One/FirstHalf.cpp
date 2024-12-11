#include <fstream>
#include <vector>
#include <iostream>
#include <tuple>
#include <cmath>
#include <bits/stdc++.h>
using namespace std;

vector<string> getData(bool test);
tuple<vector<int>, vector<int>> splitLeftAndRight(vector<string> input);
int getDistance(vector<int> left, vector<int> right);

int main() {

    // Getting the data from the file
    vector<string> data = getData(false);

    // Splitting into a left and right list
    auto [left, right] = splitLeftAndRight(data);

    // Sorting to match up each one
    sort(left.begin(), left.end());
    sort(right.begin(), right.end());

    int answer = getDistance(left, right);
    
    cout << "Your answer is " << answer << endl;


}

// To count and get the answer
int getDistance(vector<int> left, vector<int> right) {

    int output = 0;

    // Looping for each line
    for (int lineNumber = 0; lineNumber < left.size(); lineNumber++) {

        // Adding the difference
        output += abs(left[lineNumber] - right[lineNumber]);

    }

    return output;
    
}

// To split the left and right into their own lists (also to parse the numbers)
tuple<vector<int>, vector<int>> splitLeftAndRight(vector<string> input) {

    // Creating each output
    vector<int> left;
    vector<int> right;

    // Looping for every
    for (int i = 0; i < input.size(); i++) {

        // The start of the spaces
        int spaceIndex = input[i].find(" ");
        
        // The leftmost number 
        left.push_back(stoi(input[i].substr(0, spaceIndex)));
        right.push_back(stoi(input[i].substr(spaceIndex + 3)));

    }   

    return make_tuple(left, right);


}

vector<string>  getData(bool test) {

    // We need to open a file, but depending on the type we don't know which one
    if (!test) {
        ifstream input("Input.txt");
    
        // Our output
        vector<string> output;

        // Reading file
        string line;
        if (input.is_open()) {
            while (getline(input, line)){
                output.push_back(line);
            }
            input.close();
        } else {
            cout << "There was an error reading the file" << endl;
        }
        return output;
    } else {
        ifstream input("test.txt");

        // Our output
        vector<string> output;

        // Reading file
        string line;
        if (input.is_open()) {
            while (getline(input, line)){
                output.push_back(line);
            }
            input.close();
        } else {
            cout << "There was an error reading the file" << endl;
        }
        return output;
    }
}