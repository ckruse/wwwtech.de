const element = document.getElementById("title");
const slugElement = document.getElementById("slug");

if (element) {
  element.addEventListener("blur", (ev) => {
    if (!slugElement || !!slugElement.value) {
      return;
    }

    const title = ev.target.value;
    const slug = title
      .toLowerCase()
      .replaceAll(" ", "-")
      .replaceAll("ä", "ae")
      .replaceAll("ö", "oe")
      .replaceAll("ü", "ue")
      .replaceAll("ß", "ss")
      .replaceAll(/[:;,.!?()\[\]\{\}=+*\/\\|<>]/g, "");

    slugElement.value = slug;
  });
}
