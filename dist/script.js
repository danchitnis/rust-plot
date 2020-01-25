import Dygraph from "dygraphs";
import {extra} from "./extra";



    const info = document.getElementById("info");
    const pFloat = document.getElementById("float");
    info.innerHTML = "Hi!🌮";

    export function dostuff(str) {
        info.innerHTML = "";
        pFloat.innerHTML = "";
        const array = strToFloatarray(str);
        array.forEach(e => {
            pFloat.innerHTML = pFloat.innerHTML + e.toString() + "<br>";
        });
        info.innerHTML = "do stuff Done!";
        let e = new extra();
        e.hello_extra();
        //plot();

    }
    export function plot() {
        
        const g = new Dygraph(document.getElementById("myDiv"),
        [
          [1,10,100],
          [2,20,80],
          [3,50,60],
          [4,70,80]
        ],
        {
          labels: [ "x", "A", "B" ]
        });
    }

    function strToFloatarray(str) {
        const arrayLength = Math.floor(str.length / 8);
        const array = new Float32Array(arrayLength);
        let index = 0;
        for (let i = 0; i < str.length; i = i + 8) {
            const byte0 = parseInt(str[i + 0] + str[i + 1], 16);
            const byte1 = parseInt(str[i + 2] + str[i + 3], 16);
            const byte2 = parseInt(str[i + 4] + str[i + 5], 16);
            const byte3 = parseInt(str[i + 6] + str[i + 7], 16);
            const buf = new ArrayBuffer(4);
            // Create a data view of it
            const view = new DataView(buf);
            // set bytes
            view.setUint8(0, byte0);
            view.setUint8(1, byte1);
            view.setUint8(2, byte2);
            view.setUint8(3, byte3);
            const num = view.getFloat32(0);
            array[index] = num;
            index++;
        }
        return array;
    }


//export function dostuff;
/*module.exports = {
    dostuff: dostuff,
    plot: plot,
};*/