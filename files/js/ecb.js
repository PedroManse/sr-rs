window.onload = () => {
	const slct = document.querySelector("select");
	const stats = Array.from(document.querySelectorAll("fieldset.stat.send"))
	const reads = Array.from(document.querySelectorAll("fieldset.stat.get"))
	slct.addEventListener("change", ()=>{

		stats.forEach(st=>{
			st.classList.add("invisible")
			if (`write-${slct.value}` === st.id) {
				st.classList.remove("invisible")
			}
		})
		reads.forEach(rd=>{
			rd.classList.add("invisible")
			if (`read-${slct.value}` === rd.id) {
				rd.classList.remove("invisible")
			}
		})

	})
}
