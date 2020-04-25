var currentObj;
var currentTabIsProfiles = true;

function homepage() {
	document.getElementById("main-panel").innerHTML = '';
}

function cmd(commandI, valueI, string_valueI, steamI, indexI) {
	let command = {command: commandI, value: valueI, string_value: string_valueI, steam: steamI, index: indexI};
	external.invoke(JSON.stringify(command));
}

async function catchObj(jsonString) {
	currentObj = jsonString;
	display();
}

function update() {
	get();
}

function display() {
	document.getElementById("control-items").innerHTML = ""; 
	if (currentTabIsProfiles) {
		for (var i = 0; i < currentObj.profiles.length; i++) {
			document.getElementById("control-items").innerHTML += "<a class='panel-block' onclick='show(" + i + ")'>" + currentObj.profiles[i].name + "</a>"; 
		}
	} else {
		for (var i = 0; i < currentObj.steam_paths.steam_folder_paths.length; i++) {
			document.getElementById("control-items").innerHTML += "<a class='panel-block'>" + currentObj.steam_paths.steam_folder_paths[i] + "</a>"; 
		}
	}	
}

function get() {
	external.invoke('get');
}

function show(index) {
	update();
	document.getElementById("main-panel").innerHTML = '<div class="field" id="profile" style="height: 100%; width: 100%;"><div align="right"><a class="button is-info top-button">Read All from Config</a></div></div>'; 
	for (var i = 0; i < currentObj.profiles[index].game_vec.length; i++) {
		document.getElementById("profile").innerHTML += '<br><br><br><div class="control is-expanded game short-fade"><label class="label">' + currentObj.profiles[index].game_vec[i].type + '</label><p>Sensitivity Value: </p><input class="input" type="text" value="' + currentObj.profiles[index].game_vec[i].sens + '"></p><br><p class="control"><a class="button is-info game-button">Equalize</a><a class="button is-info game-button">Read from Config</a></div>'; 
	}
}

function editPath(index) {
	update();
	document.getElementById("main-panel").innerHTML = '<div class="field" id="path" style="height: 100%; width: 100%;"><a class="button is-info top-button">Read All from Config</a></div>'; 
	for (var i = 0; i < currentObj.steam_paths.length; i++) {
		document.getElementById("profile").innerHTML += '<div class="control is-expanded game short-fade"><label class="label">' + currentObj.profiles[index].game_vec[i].type + '</label><p>Sensitivity Value: </p><input class="input" type="text" value="' + currentObj.profiles[index].game_vec[i].sens + '"></p><br><p class="control"><a class="button is-info game-button">Equalize</a><a class="button is-info game-button">Read from Config</a></div><br><br><br>'; 
	}
}

function throwIt() {
	external.invoke(JSON.stringify(currentObj));
}

function eq() {
	
}

function setSens() {

}

function addProf() {

}

function addSteam() {

}

function rmProf() {

}

function rmSteam() {

}

function switchProf() {

}

function selectTab(isProfiles) {
	currentTabIsProfiles = isProfiles;
	if (isProfiles) {
		document.getElementById("profileTab").classList.add('is-active');
		document.getElementById("gameTab").classList.remove('is-active');
	} else {
		document.getElementById("profileTab").classList.remove('is-active');
		document.getElementById("gameTab").classList.add('is-active');
	}
	display();
}