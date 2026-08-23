# OBS Church Automator

> Automates setting up a church service for OBS by taking in a TXT file of the bulletin and outputting a JSON

-----------------------------------------------------

### How do I use the GUI (Graphical User Interface)?

Upon launching the program, you will be met with this screen:

<img width="624" height="263" alt="image" src="https://github.com/user-attachments/assets/0c39d5ab-19ba-49cb-a905-2e9674e70ce3" />

To begin, copy and paste the contents of a bulletin to a TXT file, then select that file with the "Choose File" button

Then the file will be loaded to the program

<img width="1920" height="287" alt="image" src="https://github.com/user-attachments/assets/e58829b4-f411-41e0-86f6-dae97b6afc45" />

To format the text, first select a paragraph

<img width="349" height="205" alt="image" src="https://github.com/user-attachments/assets/fbe320a9-c480-48e7-a74e-5ead45b82d75" />

> You can also click on the paragraph itself to select it

Then either type a number on your keyboard or select a number in the top right and hit apply

<img width="366" height="315" alt="image" src="https://github.com/user-attachments/assets/b0c7cad4-1e0c-4389-89bc-5c75e18410c2" />

As a result the orange number next to the paragraph will have changed

<img width="332" height="71" alt="image" src="https://github.com/user-attachments/assets/9232defa-cbea-425d-845a-11267fe116cd" />

When you are satisfied with the bulletin, hit "Save File"

If you want to double check your work, you can select Preview Mode and it will show you roughly what the service will look like in OBS

<img width="446" height="77" alt="image" src="https://github.com/user-attachments/assets/085431bf-d643-4f44-8dfa-694cd590984b" />

>To navigate the scenes, use the Next and Previous Scene buttons

Then in OBS import the JSON that was outputed by the program

<img width="668" height="528" alt="image" src="https://github.com/user-attachments/assets/31d41124-d05e-4b7a-b4cf-de23d36ad8af" />

<img width="1283" height="650" alt="image" src="https://github.com/user-attachments/assets/f13eb40b-70dc-477d-92e9-54472071522d" />

<img width="1257" height="630" alt="image" src="https://github.com/user-attachments/assets/7230ad4d-7359-47b2-8b33-c7ae2f54c249" />

### What do the numbers do?

|Number|Use Case|Explanation|
|-----:|:-------|:----------|
|0|For text you don't want in OBS|When constructing the JSON, text selected with 0 is skipped|
|1|For the credits|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|2|For readings|This text is wrapped to 40 characters and is inserted at the top left of the screen. For text longer than 21 lines, it automatically applies a scroll filter set to 10.0 pixels / second|
|3|For hymns|An empty scene is inserted into OBS, to add a hymn image, add an image source and select the desired hymn|
|4|For call and response (P: C:)|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|5|For empty scenes|Adds an empty scene, subsequent scenes are merged into the last one selected with 5|
|6|The name of the service|The first text selected with 6 is chosen as the name of the service. This affects the name of the JSON file, the name of the scene collection in OBS and the text in the intro slide|
|7|For text you don't want in OBS|Similarly to 0, the text is not added to OBS|
|8|For special music|This text is wrapped to 75 characters and is inserted at the top left of the screen|
|9|To add text to the previous paragraph|This will add the selected text to the paragraph selected with 1, 2 or 4 that is the closest to it going up|

> For example for 9, you would have the reading name (which is often on a separate line) be 2 and the reading body be 9
