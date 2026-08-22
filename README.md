# OBS Church Automator

> Automates setting up a church service for OBS by taking in a TXT file of the bulletin and outputting a JSON

-----------------------------------------------------

### How do I use the GUI (Graphical User Interface)?

Upon launching the program, you will be met with this screen:

<img width="2559" height="1529" alt="image" src="https://github.com/user-attachments/assets/5613a6b8-4357-429d-8920-b018ec827398" />

To begin, copy and paste the contents of a bulletin to a TXT file, then select that file with the "Choose File" button

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

Then in OBS import the JSON that was outputed by the program

<img width="668" height="528" alt="image" src="https://github.com/user-attachments/assets/31d41124-d05e-4b7a-b4cf-de23d36ad8af" />

<img width="1283" height="650" alt="image" src="https://github.com/user-attachments/assets/f13eb40b-70dc-477d-92e9-54472071522d" />

<img width="1257" height="630" alt="image" src="https://github.com/user-attachments/assets/7230ad4d-7359-47b2-8b33-c7ae2f54c249" />

### What do the numbers do?

|Number|Use Case|Explanation|
|-----:|:-------|:----------|
|0|For text you don't want in OBS|When constructing the JSON, text selected with 0 is skipped|
|1|For the credits|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|2|For readings|This text is wrapped to 40 characters and is inserted at the top left of the screen|
|3|For hymns|An empty scene is inserted into OBS, to add a hymn image, add an image source and select the desired hymn|
|4|For call and response (P: C:)|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|5|For empty scenes|Adds an empty scene, subsequent scenes are merged into the last one selected with 5|
|6|The name of the service|The first text selected with 6 is chosen as the name of the service. This affects the name of the JSON file, the name of the scene collection in OBS and the text in the intro slide|
|7|For text you don't want in OBS|Similarly to 0, the text is not added to OBS|
|8|For special music|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|9|To add text to the previous paragraph|This will add the selected text to the paragraph selected with 1, 2 or 4 that is the closest to it going up|
