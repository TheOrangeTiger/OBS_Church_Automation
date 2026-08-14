# OBS Church Automator

> Automates setting up a church service for OBS

-----------------------------------------------------

### How do I use the GUI (Graphical User Interface)?

Upon launching the program, you will be met with this screen:

<img width="2559" height="1529" alt="image" src="https://github.com/user-attachments/assets/5613a6b8-4357-429d-8920-b018ec827398" />

To begin, copy and paste the contents of a bulletin to a txt file, then select that file with the "Choose File" button

<img width="471" height="204" alt="image" src="https://github.com/user-attachments/assets/360b0151-9e77-4d97-a09c-65915dd350f2" />

Then the file will be loaded to the program

<img width="2559" height="479" alt="image" src="https://github.com/user-attachments/assets/25bbd51e-1525-4a65-9c6d-0f5af1b6f7d0" />

To format the text, first select a paragraph

<img width="652" height="271" alt="image" src="https://github.com/user-attachments/assets/a1193680-4466-4995-b14f-388a85b862b5" />

Then either type a number or select a number in the top right and hit apply

<img width="566" height="586" alt="image" src="https://github.com/user-attachments/assets/077e7c07-788c-40c3-b681-af5e719d8c6f" />

As a result the orange number next to the paragraph will have changed

<img width="2551" height="271" alt="image" src="https://github.com/user-attachments/assets/5bc82db9-a409-41da-8f4b-c8414f068c65" />

When you are satisfied with the bulletin, hit "Save File"

<img width="474" height="179" alt="image" src="https://github.com/user-attachments/assets/59649b54-ffaa-4240-aecb-0d021a477bca" />

Then in OBS import the json that was outputed by the program

<img width="668" height="528" alt="image" src="https://github.com/user-attachments/assets/31d41124-d05e-4b7a-b4cf-de23d36ad8af" />

<img width="1283" height="650" alt="image" src="https://github.com/user-attachments/assets/f13eb40b-70dc-477d-92e9-54472071522d" />

<img width="1257" height="630" alt="image" src="https://github.com/user-attachments/assets/7230ad4d-7359-47b2-8b33-c7ae2f54c249" />

### What do the numbers do?

0 and 7 Make the text not register in OBS

1, 4 and 8 Insert the text as shown in the image with the intention being that you center it yourself  
> You cannot add text to the back of an 8

<img width="1553" height="878" alt="image" src="https://github.com/user-attachments/assets/78df381e-6585-4c5e-be06-83eebe17167d" />

2 Inserts the text as shown in the image
> This is mainly intended for readings

<img width="1544" height="862" alt="image" src="https://github.com/user-attachments/assets/569f2243-f6dc-4dbf-a28d-b3528988ca47" />

3 and 5 Create an empty scene with the selected paragraph being the name
> Paragraphs selected with 5 are merged into the last consecutive one
> You are expected to insert an image of the hymn yourself with 3

6 Is the name of the service
> The name is selected based on the first paragraph labeled with 6
> If no paragraph is labeled with 6, the first paragraph is chosen

9 Adds text of the selected paragraph to a 1, 2 or 4
> For example, the name of the reading might be assigned 2 and the reading itself assigned 9

### How do I use the CLI (Command Line Interface)?

Follow the prompts and you should get the hang of it very quickly
