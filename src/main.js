const invoke = window.__TAURI__.invoke


const info = document.getElementById('info')


async function invocaDesdeTauri() {

  const desdeTauri = await invoke('hello_world', {message: 'un mensaje'}) //con invoke llamo a la funcion de rust, con su nombre y en el objeto le paso los parametros
  console.log(desdeTauri)
  return desdeTauri
}




document.getElementById('btn').addEventListener('click', async () => {info.innerHTML = await invocaDesdeTauri()})