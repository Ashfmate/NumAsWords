# Num As Words

Really simple to understand. I want to input a number, and get the words representation of the number.

# Example
- `1550` $\Rightarrow$ "one thousand five hundred and fifty"
- `-127` $\Rightarrow$ "negative one hundred and twenty seven"
- `2000` $\Rightarrow$ "two thousand"

# Requirements

- You must parse English numbers. Other languages are bonuses (More on them later)
- You must not use any external libraries, what you get from your language is what you use.
- If you would want to use a gui framework for viewing purposes (like react or flutter) then you can only use that, no plugins or packages more.
- The "and" between the hundreds and the tens/units is mandatory.
- You must put your language/gui framework of choice in a directory and call it {your username}_{language/gui framework}_num_as_words.
	- If your language/gui framework of choice has different case standards, it is best to use theirs, otherwise default to snake case
- The program has to work on any number, even negative ones.
	- Minimum integers size is four bytes. So from negative 2 billion up to positive 2 billion is required, anything above is bonus (again more on that later)
- There must be a README file in the directory to tell me how to build it, and if there might be errors, how to fix them (I don't want to abuse ChatGPT with questions) and how to run the program (There could be something done with docker but later).

IDK, maybe I will another requirement but this is what I can get

Oh and I directories that don't have a username before them means that they were done by me.

# Contribution guide

Fork this repo, add your language/gui framework as stated in here [here](#Requirements). Then submit your pull request and I will review it.

# Points

Points will be tallied for each of the participants in this manner.
- If the language of choice is a "low level" language, then it is given one point for the use of references and reasonably fast code (no need for ultra optimization).
- If the language of choice is a "high level" language, then it is given one point for being easy to read and maintain (again reasonably, no need for mega project level of design).
- If your language performs better than other participants in the field of your language's category (low level and high level like we discussed), then you get two points.
- If you language performs better than other participants in the opposite field of your language's category, then you get three points.
- Each feature you add (ones that are not from the requirements) counts as an extra point.
- Some features are very insignificant and as such may not get a point. For example, I add a comma between each three numbers, but this is not really an achievement and as such, will not count towards my points.
- GUI applications already count as an extra point as it means they have to 

## Esoteric Languages

If you are using an esoteric language then points for readability is already off, the points added will have to be something that is quirky about the language, so for example BQN or APL will have which one uses the least amount of characters, or emojilang being how funny it is and such.

Weird esoteric langauges are not gonna have points but you can brag if your application outperforms low level languages 😉

# Leaderboard

This readme file will contain the leaderboard of the top ten best performing users with their language/gui framework and a small list of features they applied.

No | User | Language | Features | Points
:-: | :-: | :-: | :-- | --:
1 |Ashfmate | Rust | Reasonably fast and uses monadic functions so readable | 3

___

See if you have the skills to turn what we already do intuitively as humans and apply it in programming. I believe that is the best form of programming and it is one you would definitely learn much from.
