if (window.IntersectionObserver && window.fetch) {
  let page = 0;

  function handleObserve(events) {
    const event = events[0];
    const element = document.querySelector("ol.picture-list");

    if (!event || !event.isIntersecting || !element) {
      return;
    }

    page += 1;

    fetch(`/pictures/scrolling?p=${page}`, {
      mode: "same-origin",
      credentials: "same-origin",
    })
      .then((rsp) => {
        if (!rsp.ok) {
          return;
        }

        return rsp.text();
      })
      .then((data) => {
        const fragment = document.createElement("ol");
        fragment.innerHTML = data;

        Array.from(fragment.childNodes).forEach((child) => element.appendChild(child));
      });
  }

  const observer = new IntersectionObserver(handleObserve, {
    root: null,
    rootMargin: "0px",
    threshold: 0,
  });

  observer.observe(document.querySelector(".pagination"));
}
