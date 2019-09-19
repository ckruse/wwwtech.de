import "../css/app.scss";

import Prism from "prismjs";
// import socket from "./socket"

document.addEventListener("DOMContentLoaded", e => {
  var styles = document.querySelectorAll("link[media='none']");
  styles.forEach(el => {
    el.setAttribute("media", "all");
  });

  Prism.highlightAll();
});

import "./widget";

/* eof */
