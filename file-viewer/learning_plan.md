# Learning Order

Learning steps:
1. "Hello Terminal": learning about raw mode, reading key events
2. "Cursor Dancer": learning more about screen control, and cursor
3. "File Printer": combining step 1 & 2
4. "Scrolling File Viewer"

## Step 1 - "Hello Terminal" 

Write a program that: 
1. Enables raw mode
2. Reads key events and prints them
3. Exits when you press Q
4. Restores normal mode on exit

## Step 2 - "Cursor Dancer"

Write a program that:
1. Clears the screen
2. Moves the cursor to different positions based on arrow keys
3. Prints something at each position 

## Step 3 - "File Printer"

Write a program that:
1. Takes a file path as an argument
2. Reads the file
3. Clears the screen and prints the file content from the top
4. Gets terimnal size and only prints what fits

## Step 4 - "Scrolling File Viewer"

Combine everything:
1. Arrow keys scroll up/down through the file
2. Terminal resize events redraw properly
3. q quits cleanly
4. Line numbers on the left (optional bonus)