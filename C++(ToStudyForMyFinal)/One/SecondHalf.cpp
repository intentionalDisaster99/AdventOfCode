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
int count(vector<int> list, int target);

int main() {

    // Getting the data from the file
    vector<string> data = getData(false);

    // Splitting into a left and right list
    auto [left, right] = splitLeftAndRight(data);

    // Sorting to match up each one
    sort(left.begin(), left.end());
    sort(right.begin(), right.end());

    int answer = 0;

    // Adding similarity 
    for (int i = 0; i < left.size(); i++) {
        answer += count(right, left[i]) * left[i];

    }
    
    cout << "Your answer is " << answer << endl;


}

// A binary search! 
int count(vector<int> list, int target) {

    // We basically will have to binary search until we find one and then count the number that are there
    int high = list.size()-1;
    int low = 0;
    int mid = list.size() / 2;

    while (list[mid] != target) {

        if (target < list[mid]) {
            high = mid;
        } else if (target != list[mid]){
            // It also doesn't exist if low == mid here, cause then it don't work
            if (low == mid) {
                return 0;
            }

            low = mid;
        } else {
            break;
        }

        mid = (high + low) / 2;        

        // If the high and the low are the same, then there isn't one 
        if (high == low) {
            return 0;
        }

        
    }

    // The answer and a counting variable
    int output = 1;
    int index = mid;

    // Counting the ones above the input
    while (list[index + 1] == target) {
        // Making it memory safe
        if (index + 1 >= list.size()) {
            break;
        }
        output++;
        index++;
    }
    // Counting the ones blow the input
    while (list[mid - 1] == target) {
        // Making it memory safe
        if (mid - 1 < 0) {
            break;
        }
        output++;
        mid--;
    }

    return output;
    
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