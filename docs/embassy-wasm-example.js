let Q=0,K=`undefined`,U=`Object`,X=14,O=`utf-8`,S=`string`,R=1,N=null,W=4,T=`function`,M=Array,P=Error,V=FinalizationRegistry,Z=Object,Y=Object.getPrototypeOf,L=undefined;var C=(()=>{if(B===N||B.byteLength===Q){B=new Uint8ClampedArray(b.memory.buffer)};return B});var u=((a,c,d,e)=>{const f={a:a,b:c,cnt:R,dtor:d};const g=(...a)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...a)}finally{if(--f.cnt===Q){b.__wbindgen_export_2.get(f.dtor)(c,f.b);t.unregister(f)}else{f.a=c}}};g.original=f;t.register(g,f,f);return g});function z(a,c){try{return a.apply(this,c)}catch(a){b.__wbindgen_exn_store(l(a))}}var A=(a=>a===L||a===N);var d=(a=>c[a]);var m=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==N){return `${a}`};if(b==S){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==N){return `Symbol`}else{return `Symbol(${b})`}};if(b==T){const b=a.name;if(typeof b==S&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>Q){c+=m(a[Q])};for(let d=R;d<b;d++){c+=`, `+ m(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>R){d=c[R]}else{return toString.call(a)};if(d==U){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return U}};if(a instanceof P){return `${a.name}: ${a.message}\n${a.stack}`};return d});var J=(async(a)=>{if(b!==L)return b;if(typeof a!==K&&Y(a)===Z.prototype)({module_or_path:a}=a);else console.warn(`using deprecated parameters for the initialization function; pass a single object instead`);if(typeof a===K){a=new URL(`embassy-wasm-example_bg.wasm`,import.meta.url)};const c=F();if(typeof a===S||typeof Request===T&&a instanceof Request||typeof URL===T&&a instanceof URL){a=fetch(a)};G(c);const {instance:d,module:e}=await E(await a,c);return H(d,e)});var j=(()=>{if(i===N||i.byteLength===Q){i=new Uint8Array(b.memory.buffer)};return i});var I=(a=>{if(b!==L)return b;if(typeof a!==K&&Y(a)===Z.prototype)({module:a}=a);else console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`);const c=F();G(c);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const d=new WebAssembly.Instance(a,c);return H(d,a)});var G=((a,b)=>{});var f=(a=>{if(a<132)return;c[a]=e;e=a});var s=(()=>{if(r===N||r.buffer.detached===!0||r.buffer.detached===L&&r.buffer!==b.memory.buffer){r=new DataView(b.memory.buffer)};return r});var F=(()=>{const c={};c.wbg={};c.wbg.__wbg_instanceof_DomException_1bbe86882eadb549=(a=>{let b;try{b=d(a) instanceof DOMException}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_newwithu8clampedarray_6b29095634b7e758=function(){return z(((a,b,c)=>{const d=new ImageData(D(a,b),c>>>Q);return l(d)}),arguments)};c.wbg.__wbg_putImageData_d8c261486f99879a=function(){return z(((a,b,c,e)=>{d(a).putImageData(d(b),c,e)}),arguments)};c.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});c.wbg.__wbg_resolve_570458cb99d56a43=(a=>{const b=Promise.resolve(d(a));return l(b)});c.wbg.__wbg_then_95e6edc0f89b73b1=((a,b)=>{const c=d(a).then(d(b));return l(c)});c.wbg.__wbg_clearTimeout_dce8f348fe8a8957=typeof clearTimeout==T?clearTimeout:y(`clearTimeout`);c.wbg.__wbg_setTimeout_6afbbe8f39b1c9ad=((a,b)=>{const c=setTimeout(d(a),b>>>Q);return c});c.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return l(c)});c.wbg.__wbg_error_09480e4aadca50ad=(a=>{console.error(d(a))});c.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==R){b.a=Q;return !0};const c=!1;return c});c.wbg.__wbg_createElement_5921e9eb06b9ec89=function(){return z(((a,b,c)=>{const e=d(a).createElement(k(b,c));return l(e)}),arguments)};c.wbg.__wbg_instanceof_HtmlCanvasElement_1a96a01603ec2d8b=(a=>{let b;try{b=d(a) instanceof HTMLCanvasElement}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_setwidth_e371a8d6b16ebe84=((a,b)=>{d(a).width=b>>>Q});c.wbg.__wbg_setheight_ba99ad2df4295e89=((a,b)=>{d(a).height=b>>>Q});c.wbg.__wbg_body_b3bb488e8e54bf4b=(a=>{const b=d(a).body;return A(b)?Q:l(b)});c.wbg.__wbg_instanceof_Element_cc034878d52a64fa=(a=>{let b;try{b=d(a) instanceof Element}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_appendChild_ac45d1abddf1b89b=function(){return z(((a,b)=>{const c=d(a).appendChild(d(b));return l(c)}),arguments)};c.wbg.__wbg_getContext_69ec873410cbba3c=function(){return z(((a,b,c)=>{const e=d(a).getContext(k(b,c));return A(e)?Q:l(e)}),arguments)};c.wbg.__wbg_instanceof_CanvasRenderingContext2d_a0c4f0da6392b8ca=(a=>{let b;try{b=d(a) instanceof CanvasRenderingContext2D}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_start_060c0a26c901f6b8=function(){return z((a=>{d(a).start()}),arguments)};c.wbg.__wbg_code_4a4b2516783729c7=(a=>{const b=d(a).code;return b});c.wbg.__wbg_stop_43038ebfd9ce25a7=function(){return z((a=>{d(a).stop()}),arguments)};c.wbg.__wbg_classList_d725bcb3b32c27b5=(a=>{const b=d(a).classList;return l(b)});c.wbg.__wbg_remove_0dd2beafdaa4d9ba=function(){return z(((a,b,c)=>{d(a).remove(k(b,c))}),arguments)};c.wbg.__wbg_add_e210e3b838bff57f=function(){return z(((a,b,c)=>{d(a).add(k(b,c))}),arguments)};c.wbg.__wbg_new_4e9fd42b8fa0b088=function(){return z((()=>{const b=new a();return l(b)}),arguments)};c.wbg.__wbg_createOscillator_e8dbe966fe1b7b25=function(){return z((a=>{const b=d(a).createOscillator();return l(b)}),arguments)};c.wbg.__wbg_createGain_cb4f1f2157ffa8f5=function(){return z((a=>{const b=d(a).createGain();return l(b)}),arguments)};c.wbg.__wbg_settype_35ce6ce0a97c0058=((a,b)=>{d(a).type=[`sine`,`square`,`sawtooth`,`triangle`,`custom`][b]});c.wbg.__wbg_destination_02fda856cc855541=(a=>{const b=d(a).destination;return l(b)});c.wbg.__wbg_frequency_8889aec8b4f6abc8=(a=>{const b=d(a).frequency;return l(b)});c.wbg.__wbg_setvalue_1ceecec803e3b851=((a,b)=>{d(a).value=b});c.wbg.__wbindgen_debug_string=((a,c)=>{const e=m(d(c));const f=q(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=n;s().setInt32(a+ W*R,g,!0);s().setInt32(a+ W*Q,f,!0)});c.wbg.__wbindgen_throw=((a,b)=>{throw new P(k(a,b))});c.wbg.__wbg_performance_fa12dc8712926291=(a=>{const b=d(a).performance;return A(b)?Q:l(b)});c.wbg.__wbg_now_a69647afb1f66247=(a=>{const b=d(a).now();return b});c.wbg.__wbg_document_8554450897a855b9=(a=>{const b=d(a).document;return A(b)?Q:l(b)});c.wbg.__wbg_getElementById_f56c8e6a15a6926d=((a,b,c)=>{const e=d(a).getElementById(k(b,c));return A(e)?Q:l(e)});c.wbg.__wbg_addEventListener_e167f012cbedfa4e=function(){return z(((a,b,c,e)=>{d(a).addEventListener(k(b,c),d(e))}),arguments)};c.wbg.__wbg_connect_9ef7bb6259c61b23=function(){return z(((a,b)=>{const c=d(a).connect(d(b));return l(c)}),arguments)};c.wbg.__wbg_self_3093d5d1f7bcb682=function(){return z((()=>{const a=self.self;return l(a)}),arguments)};c.wbg.__wbg_window_3bcfc4d31bc012f8=function(){return z((()=>{const a=window.window;return l(a)}),arguments)};c.wbg.__wbg_globalThis_86b222e13bdf32ed=function(){return z((()=>{const a=globalThis.globalThis;return l(a)}),arguments)};c.wbg.__wbg_global_e5a3fe56f8be9485=function(){return z((()=>{const a=global.global;return l(a)}),arguments)};c.wbg.__wbindgen_is_undefined=(a=>{const b=d(a)===L;return b});c.wbg.__wbg_newnoargs_76313bd6ff35d0f2=((a,b)=>{const c=new Function(k(a,b));return l(c)});c.wbg.__wbg_call_1084a111329e68ce=function(){return z(((a,b)=>{const c=d(a).call(d(b));return l(c)}),arguments)};c.wbg.__wbindgen_object_clone_ref=(a=>{const b=d(a);return l(b)});c.wbg.__wbg_instanceof_Window_5012736c80a01584=(a=>{let b;try{b=d(a) instanceof Window}catch(a){b=!1}const c=b;return c});c.wbg.__wbindgen_closure_wrapper73=((a,b,c)=>{const d=u(a,b,X,v);return l(d)});c.wbg.__wbindgen_closure_wrapper80=((a,b,c)=>{const d=u(a,b,X,v);return l(d)});c.wbg.__wbindgen_closure_wrapper143=((a,b,c)=>{const d=u(a,b,33,w);return l(d)});return c});var l=(a=>{if(e===c.length)c.push(c.length+ R);const b=e;e=c[b];c[b]=a;return b});var g=(a=>{const b=d(a);f(a);return b});var E=(async(a,b)=>{if(typeof Response===T&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===T){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var H=((a,c)=>{b=a.exports;J.__wbindgen_wasm_module=c;r=N;i=N;B=N;b.__wbindgen_start();return b});var q=((a,b,c)=>{if(c===L){const c=o.encode(a);const d=b(c.length,R)>>>Q;j().subarray(d,d+ c.length).set(c);n=c.length;return d};let d=a.length;let e=b(d,R)>>>Q;const f=j();let g=Q;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==Q){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,R)>>>Q;const b=j().subarray(e+ g,e+ d);const f=p(a,b);g+=f.written;e=c(e,d,g,R)>>>Q};n=g;return e});var D=((a,b)=>{a=a>>>Q;return C().subarray(a/R,a/R+ b)});var k=((a,b)=>{a=a>>>Q;return h.decode(j().subarray(a,a+ b))});var y=(a=>()=>{throw new P(`${a} is not defined`)});var w=((a,c)=>{b._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcfbdb5982220b5ab(a,c)});var v=((a,c,d)=>{b._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9a2b8ff1aafbe917(a,c,l(d))});const a=typeof AudioContext!==K?AudioContext:(typeof webkitAudioContext!==K?webkitAudioContext:L);let b;const c=new M(128).fill(L);c.push(L,N,!0,!1);let e=c.length;const h=typeof TextDecoder!==K?new TextDecoder(O,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw P(`TextDecoder not available`)}};if(typeof TextDecoder!==K){h.decode()};let i=N;let n=Q;const o=typeof TextEncoder!==K?new TextEncoder(O):{encode:()=>{throw P(`TextEncoder not available`)}};const p=typeof o.encodeInto===T?((a,b)=>o.encodeInto(a,b)):((a,b)=>{const c=o.encode(a);b.set(c);return {read:a.length,written:c.length}});let r=N;const t=typeof V===K?{register:()=>{},unregister:()=>{}}:new V(a=>{b.__wbindgen_export_2.get(a.dtor)(a.a,a.b)});function x(){b.main()}let B=N;export default J;export{x as main,I as initSync}