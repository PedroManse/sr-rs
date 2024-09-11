function showSome(hides, show) {
	hides.forEach(hide=>{
		hide.classList.add("invisible")
		if (show === hide.id) {
			hide.classList.remove("invisible")
		}
	})
}

window.onload = () => {
	const types = Array.from(document.querySelectorAll("input[type=radio]"))
	const stats = Array.from(document.querySelectorAll("fieldset.stat.send"))
	const reads = Array.from(document.querySelectorAll("fieldset.stat.get"))

	types.forEach((clipType)=>{
		clipType.addEventListener("change", ()=>{
			showSome(stats, `write-${clipType.value}`)
			showSome(reads, `read-${clipType.value}`)
		})
		// show correct tab in case of soft refresh
		if (clipType.checked) {
			showSome(stats, `write-${clipType.value}`)
			showSome(reads, `read-${clipType.value}`)
		}
	})

}
