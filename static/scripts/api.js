const get_token = () => {
  return window.localStorage.getItem("token")
}

const set_token = (token) => {
  window.localStorage.setItem("token", token)
}

const clear_token = (token) => {
  window.localStorage.removeItem("token")
}

const post = (path, body) =>
  fetch("/api/" + path, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + get_token()
    },
    body: JSON.stringify(body)
  });
const put = (path, body) =>
    fetch("/api/" + path, {
      method: "PUT",
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer ' + get_token()
      },
      body: JSON.stringify(body)
    });

const get = (path) =>
  fetch("/api/" + path, {
    method: "GET", headers: { 'Authorization': 'Bearer ' + get_token() }
  })
    .then(r => r.json());

const refresh_token = () => {
  return get("refresh").then(j => { set_token(j.token) }).catch(e => {
    console.log(e);
    clear_token();
    if (!window.location.pathname.endsWith("login") && !window.location.pathname.endsWith("register")) {
      window.location = "/login";
    }
    throw new Error("Refresh failed: " + e);
  })
}

const form_submit = (callback) => window.addEventListener("load", () => document.querySelector("form").addEventListener("submit", callback));
const update_field = (name, data) => document.querySelector("#"+name).value = data[name];
const get_field = (name) => document.querySelector("#"+name).value;