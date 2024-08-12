window.onload = () => {
	const types = Array.from(document.querySelectorAll("input[type=radio]"))
	const stats = Array.from(document.querySelectorAll("fieldset.stat.send"))
	const reads = Array.from(document.querySelectorAll("fieldset.stat.get"))

	types.forEach((clipType)=>{
		clipType.addEventListener("change", ()=>{
			stats.forEach(st=>{
				st.classList.add("invisible")
				if (`write-${clipType.value}` === st.id) {
					st.classList.remove("invisible")
				}
			})
			reads.forEach(rd=>{
				rd.classList.add("invisible")
				if (`read-${clipType.value}` === rd.id) {
					rd.classList.remove("invisible")
				}
			})
		})
	})

}
