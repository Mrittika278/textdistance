
/* TextDistance-RS Premium Frontend Script */

const $ = (id)=>document.getElementById(id);

const ui={
 string1:$("string1"),
 string2:$("string2"),
 algorithm:$("algorithm"),
 button:$("calculateBtn"),
 loading:$("loading"),
 algorithmName:$("algorithmName"),
 distance:$("distance"),
 similarity:$("similarity"),
 execution:$("execution"),
 status:$("status")
};

document.addEventListener("DOMContentLoaded",()=>{
  ui.button.addEventListener("click",calculateDistance);
  document.addEventListener("keydown",e=>{
    if(e.key==="Enter") calculateDistance();
  });
});

async function calculateDistance(){

 const s1=ui.string1.value.trim();
 const s2=ui.string2.value.trim();
 const algo=ui.algorithm.value;

 if(!s1||!s2){
   showToast("Please enter both strings.","warning");
   return;
 }

 if(algo==="hamming" && s1.length!==s2.length){
   showToast("Hamming Distance requires equal-length strings.","error");
   return;
 }

 startLoading();

// Record when loading started
const startTime = Date.now();

try {

    const response = await fetch("/api/calculate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            algorithm: algo,
            string1: s1,
            string2: s2
        })
    });

    if (!response.ok) {
        throw new Error("Backend request failed.");
    }

    const data = await response.json();

    // Keep loading animation visible for at least 1 second
    const elapsed = Date.now() - startTime;

    if (elapsed < 1000) {
        await new Promise(resolve =>
            setTimeout(resolve, 1000 - elapsed)
        );
    }

    
    update(ui.algorithmName, beautify(data.algorithm));

update(ui.distance, data.distance ?? "-");

update(
    ui.similarity,
    data.similarity != null
        ? (data.similarity * 100).toFixed(2) + "%"
        : "-"
);

update(
    ui.execution,
    Number(data.execution_ms).toFixed(3) + " ms"
);

update(ui.status, "✅ Behavioral Equivalent");

successAnimation();

}
catch(err){

    console.error(err);

    ui.status.innerHTML = "❌ Failed";

    showToast(err.message,"error");

}
finally{

    stopLoading();

}

}

function update(el,val){
 el.style.opacity=0;
 el.style.transform="translateY(12px)";
 setTimeout(()=>{
   el.textContent=val;
   el.style.transition=".35s";
   el.style.opacity=1;
   el.style.transform="translateY(0)";
 },150);
}

function beautify(a){
 return {
   levenshtein:"Levenshtein",
   damerau:"Damerau-Levenshtein",
   hamming:"Hamming",
   jaro:"Jaro",
   jaro_winkler:"Jaro-Winkler",
   jaccard:"Jaccard",
   cosine:"Cosine"
 }[a] || a;
}

function startLoading(){
 ui.loading.classList.remove("hidden");
 ui.button.disabled=true;
 ui.button.innerHTML="Running Rust Engine...";
}

function stopLoading(){
 ui.loading.classList.add("hidden");
 ui.button.disabled=false;
 ui.button.innerHTML="🦀 Calculate Distance";
}

function successAnimation(){

    ui.button.animate(
        [
            { transform: "scale(1)" },
            { transform: "scale(1.05)" },
            { transform: "scale(1)" }
        ],
        {
            duration: 400,
            easing: "ease-in-out"
        }
    );

}

function showToast(msg,type="success"){
 const t=document.createElement("div");
 t.textContent=msg;
 Object.assign(t.style,{
   position:"fixed",top:"24px",right:"24px",
   padding:"16px 22px",borderRadius:"14px",
   color:"#fff",fontWeight:"600",zIndex:9999,
   boxShadow:"0 20px 40px rgba(0,0,0,.2)"
 });
 t.style.background=
   type==="error"?"#dc2626":
   type==="warning"?"#d97706":"#16a34a";
 document.body.appendChild(t);
 setTimeout(()=>{t.style.opacity="0";},2500);
 setTimeout(()=>t.remove(),3000);
}
