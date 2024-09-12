const $ = (q,doc=document) => doc.querySelector(q)
const $$ =(qs,doc=document)=> Array.from(doc.querySelectorAll(qs))

let _color_counter = 0;
const _auto_colors = [
	["#45938B", "#000000"],
	["#77CCBB", "#000000"],
	["#A8F7ED", "#000000"],
	["#BBBBBB", "#000000"],
	["#EEEEEE", "#000000"],
	["#EEAA66", "#000000"],
	["#BB7733", "#000000"],
	["#FF0000", "#000000"],
	["#FFFF00", "#000000"],
];
const autoColor=()=>_auto_colors[(_color_counter++)%_auto_colors.length];

function El(name, value="", attributes=null) {
	if (typeof (value) == "object" && !Array.isArray(value)) {
		[value, attributes] = [attributes, value];
		if (value === null) value = "";
	}
	const el = document.createElement(name);
	for (const attr in attributes) {
		if (attr === "style") {
			const stlyAtrs = Object.keys(attributes.style);
			for (const stl of stlyAtrs) {
				el.style[stl]  = attributes.style[stl];
			}
		} else {
			el.setAttribute(attr, attributes[attr]);
		}
	}
	if (typeof(value) === "string") {
		el.innerText = value;
	} else if (Array.isArray(value)) {
		value.forEach( el.appendChild.bind(el) )
	}
	return el;
}

function savableNote(card) {
	const saveButton = $("button", card);
	const noteid = card.getAttribute("noteid")
	const deleteNoteButton = El("button", {class: "edit-delete"}, "🗑")
	card.prepend(deleteNoteButton);
	saveButton.innerText="💾";
	saveButton.classList.toggle("edit-save");
	deleteNoteButton.addEventListener("click", async ()=>{
		if (!confirm("Deletar nota?")) {
			return;
		}
		const r = await (fetch("/meet/user/note/"+noteid, {
			method: "DELETE",
		}).then(
			a=>a.status,
			()=>600
		))
		if (r === 200) {
			card.remove();
		} else {
			savableNote(card)
		}
	})

	saveButton.addEventListener("click", async ()=>{
		const editArea = $("textarea", card);
		const content = editArea.value;
		const contentArea = El("span", content);

		const r = await (fetch("/meet/user/note/"+noteid, {
			method: "PUT",
			headers: {"Content-Type":"application/json"},
			body: JSON.stringify({content})
		}).then(
			a=>a.status,
			()=>600
		))

		if (r === 200) {
			deleteNoteButton.remove();
			editArea.remove();
			card.prepend(contentArea);
			editableNote(card)
		} else {
			savableNote(card)
		}
	}, {once: true})
}

function editableNote(card) {
	const editButton = $("button", card);
	editButton.innerText="✎"
	editButton.addEventListener("click", ()=>{
		const content = $("span", card).innerText;
		const editArea = El("textarea", content);
		editArea.focus();
		console.log(content, editArea)
		$("span", card).remove()
		card.prepend(editArea)
		savableNote(card)
	}, {once: true})
}

function makeNote({id, content}) {
	const [bkcolor, txcolor] = autoColor();
	const card = El("div", {
		noteid: id,
		class: "note",
		style: {
			"background-color": bkcolor,
			"color": txcolor,
		},
	}, [
		El("button", {class:"edit", type: "button"}),
		El("span", content),
	])
	editableNote(card)
	return card;
}

function makeNoteCreator(noteBoard) {
	const button = El("button", {type: "button", title: "create"}, "+");
	const textArea = El("textarea");
	const card = El("div", [ textArea, button ]);
	button.addEventListener("click", async ()=>{
		const content = textArea.value;
		const r = await fetch("/meet/user/note", {
			method: "POST",
			headers: {"Content-Type":"application/json"},
			body: JSON.stringify({content})
		}).catch(()=>({status: 400}));
		if (r.status === 200) {
			const note = await r.json();
			textArea.value = ""
			const noteCard = makeNote(note)
			noteBoard.appendChild(noteCard)
		} else {
			alert("failed to create note")
		}
	})
	return card;
}


function newVisilibityObserver(callback) {
	return new IntersectionObserver((entries, observer) => {
		entries
			.filter(e=>e.intersectionRatio > 0)
			.forEach(callback);
	}, { root: document.documentElement });
}

function respondToVisibility(element, callback) {
	newVisilibityObserver(callback).observe(element);
}

function listTriggerNote(noteBoard) {
	const loader = El("img", {src: "https://htmx.org/img/bars.svg"});
	noteBoard.appendChild(loader)
	let page = 0;
	const page_size = 15;
	respondToVisibility(loader, async ()=>{
		const query = new URLSearchParams({page, page_size});
		page++;
		fetch("/meet/user/note?"+query)
			.then(a=>a.json())
			.then(notes=>{
				notes.content
					.map(makeNote)
					.forEach(e=>noteBoard.insertBefore(e, loader))
				if (notes.taken < page_size) {
					loader.remove();
				}
			})
	})
	return loader;
}

async function initNoteSystem() {
	const noteBoard = $("#notes")
	const noteContainer = $("#note-container", noteBoard);
	{ // note creator
		const noteCreator = makeNoteCreator(noteContainer)
		const noteCreatorContainer = $("#note-creator-container", noteBoard);
		noteCreatorContainer.appendChild(noteCreator)
		addEventListener("visibilitychange", (event) => {});
	}

	const noteLoaderContainer = $("#note-loader-container", noteBoard);
	listTriggerNote(noteContainer)
}

async function initCalendarSystem() {
}
async function initGroupSystem() {
}

window.onload = async () => {
	await initNoteSystem();
	await initCalendarSystem();
	await initGroupSystem();
}

