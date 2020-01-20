console.log("Hi from js file");

let info = document.getElementById("info");
let pFloat = document.getElementById("float");
info.innerHTML = "Hi!🌮";

function dostuff(str) {
    
    info.innerHTML = "";
    pFloat.innerHTML = "";

    let array = str_to_floatarray(str);

    array.forEach(e => {
        pFloat.innerHTML = pFloat.innerHTML +  e.toString() + "<br>";    
    });

    info.innerHTML = "Done!"
    

}

function str_to_floatarray(str) {
    
    array_length = Math.floor(str.length/8);
    array = new Float32Array(array_length);

    let index = 0;

    for (let i=0; i<str.length; i=i+8) {
        let byte0 = parseInt(str[i+0] + str[i+1], 16);
        let byte1 = parseInt(str[i+2] + str[i+3], 16);
        let byte2 = parseInt(str[i+4] + str[i+5], 16);
        let byte3 = parseInt(str[i+6] + str[i+7], 16);

        var buf = new ArrayBuffer(4);
        // Create a data view of it
        let view = new DataView(buf);

        // set bytes
        view.setUint8(0, byte0);
        view.setUint8(1, byte1);
        view.setUint8(2, byte2);
        view.setUint8(3, byte3);

        let num = view.getFloat32(0);

        array[index] = num;
        index++;
    }

    return array;

}
   