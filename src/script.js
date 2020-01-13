console.log("Hi from js file");

let info = document.getElementById("info");
let pFloat = document.getElementById("float");
info.innerHTML = "Hi!🌮";

function dostuff(str) {
    
    info.innerHTML = "";
    pFloat.innerHTML = "";

    for (let i=0; i<str.length; i=i+8) {
        let data1 = parseInt(str[i+0] + str[i+1], 16);
        let data2 = parseInt(str[i+2] + str[i+3], 16);
        let data3 = parseInt(str[i+4] + str[i+5], 16);
        let data4 = parseInt(str[i+6] + str[i+7], 16);



        info.innerHTML = info.innerHTML + data1.toString(16) + ", ";
        pFloat.innerHTML = pFloat.innerHTML +  toFloat(data1, data2, data3, data4).toString() + "<br>";    
    }

}

function toFloat(a0, a1, a2, a3) {
    
    //let data =  [65, 226, 157, 10];
    var buf = new ArrayBuffer(4);
    // Create a data view of it
    let view = new DataView(buf);

    // set bytes
    view.setUint8(0, a0);
    view.setUint8(1, a1);
    view.setUint8(2, a2);
    view.setUint8(3, a3);

    /*data.forEach(function (b, i) {
        view.setUint8(i, b);
    });*/
    //view.setUint32(0, char);

    // Read the bits as a float; note that by doing this, we're implicitly
    // converting it from a 32-bit float into JavaScript's native 64-bit double
    let num = view.getFloat32(0);
    // Done
    console.log(num);
    return num;
}