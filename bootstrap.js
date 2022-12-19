/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/libnoentiendo_bg.wasm": function() {
/******/ 			return {
/******/ 				"./libnoentiendo_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_alert_4ec518553ecb260d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_alert_4ec518553ecb260d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_prompt_cd54bd8801e664bd": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_prompt_cd54bd8801e664bd"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_number_new": function(p0f64) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_number_new"](p0f64);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGl2RenderingContext_fcfa91cd777063f3": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_instanceof_WebGl2RenderingContext_fcfa91cd777063f3"](p0i32);
/******/ 					},
/******/ 					"__wbg_beginQuery_909ec673d606f873": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_beginQuery_909ec673d606f873"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindBufferRange_b8f6dc19661d5cf7": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindBufferRange_b8f6dc19661d5cf7"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_bindSampler_4b0e0e598e2cae44": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindSampler_4b0e0e598e2cae44"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindVertexArray_9d12800e272184b0": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindVertexArray_9d12800e272184b0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_blitFramebuffer_cdc1ebf043046b70": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blitFramebuffer_cdc1ebf043046b70"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_bufferData_6ce28904b25c8be9": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferData_6ce28904b25c8be9"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bufferData_8d206d7adf6751c0": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferData_8d206d7adf6751c0"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bufferSubData_0e04c6c7fec3c949": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferSubData_0e04c6c7fec3c949"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_clearBufferfi_92173f77d7147a2f": function(p0i32,p1i32,p2i32,p3f32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_clearBufferfi_92173f77d7147a2f"](p0i32,p1i32,p2i32,p3f32,p4i32);
/******/ 					},
/******/ 					"__wbg_clearBufferfv_5cc4edeacbcf72e8": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_clearBufferfv_5cc4edeacbcf72e8"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_clearBufferiv_8bb0c2b97eedc22b": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_clearBufferiv_8bb0c2b97eedc22b"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_clearBufferuiv_1f5c5e9baa9a3d9b": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_clearBufferuiv_1f5c5e9baa9a3d9b"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_clientWaitSync_ad323ab9e423d0cf": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_clientWaitSync_ad323ab9e423d0cf"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_compressedTexSubImage2D_5b2a7dc8dc7b3e73": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compressedTexSubImage2D_5b2a7dc8dc7b3e73"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_compressedTexSubImage2D_fd1cef4f6a5da5c3": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compressedTexSubImage2D_fd1cef4f6a5da5c3"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32);
/******/ 					},
/******/ 					"__wbg_compressedTexSubImage3D_0df5a8ddb9ebafc2": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compressedTexSubImage3D_0df5a8ddb9ebafc2"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32);
/******/ 					},
/******/ 					"__wbg_compressedTexSubImage3D_9c916feb243112db": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compressedTexSubImage3D_9c916feb243112db"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_copyBufferSubData_11187dccce72b79b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_copyBufferSubData_11187dccce72b79b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_copyTexSubImage3D_7a262558a6a33f2e": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_copyTexSubImage3D_7a262558a6a33f2e"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_createSampler_288fd761eabe283d": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createSampler_288fd761eabe283d"](p0i32);
/******/ 					},
/******/ 					"__wbg_createVertexArray_8467a75e68fec199": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createVertexArray_8467a75e68fec199"](p0i32);
/******/ 					},
/******/ 					"__wbg_deleteQuery_77a7ae09eda297e1": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteQuery_77a7ae09eda297e1"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteSampler_ec3ca2243d8cfcad": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteSampler_ec3ca2243d8cfcad"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteSync_48aed3df05f4f497": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteSync_48aed3df05f4f497"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteVertexArray_00194a31d79df7e5": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteVertexArray_00194a31d79df7e5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawArraysInstanced_951a1d7e32c4f855": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawArraysInstanced_951a1d7e32c4f855"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_drawBuffers_23c1572f12f90db2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawBuffers_23c1572f12f90db2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawElementsInstanced_2e05a96af17fe284": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawElementsInstanced_2e05a96af17fe284"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_endQuery_05baee8fc782e5f0": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_endQuery_05baee8fc782e5f0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_fenceSync_91d72c970c880844": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_fenceSync_91d72c970c880844"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_framebufferTextureLayer_d5e78fc74b8261e3": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_framebufferTextureLayer_d5e78fc74b8261e3"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_getBufferSubData_6b00169c609c16f7": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getBufferSubData_6b00169c609c16f7"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_getIndexedParameter_d4a2b68e14a022a1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getIndexedParameter_d4a2b68e14a022a1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getQueryParameter_358ea490fb85e05c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getQueryParameter_358ea490fb85e05c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getSyncParameter_ab2f9499a91faae0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getSyncParameter_ab2f9499a91faae0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getUniformBlockIndex_a6f3a994dcc7399d": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getUniformBlockIndex_a6f3a994dcc7399d"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_invalidateFramebuffer_802e38619851791e": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_invalidateFramebuffer_802e38619851791e"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_readBuffer_f20d42ed12643534": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_readBuffer_f20d42ed12643534"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_readPixels_e855be1f94815442": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_readPixels_e855be1f94815442"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32);
/******/ 					},
/******/ 					"__wbg_readPixels_5d4e6205291096f0": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_readPixels_5d4e6205291096f0"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32);
/******/ 					},
/******/ 					"__wbg_renderbufferStorageMultisample_3e76453eed60554b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_renderbufferStorageMultisample_3e76453eed60554b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_samplerParameterf_6eda655d7213cb18": function(p0i32,p1i32,p2i32,p3f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_samplerParameterf_6eda655d7213cb18"](p0i32,p1i32,p2i32,p3f32);
/******/ 					},
/******/ 					"__wbg_samplerParameteri_390f1debfe40f83b": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_samplerParameteri_390f1debfe40f83b"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texStorage2D_d25a76ad1b1ea98f": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texStorage2D_d25a76ad1b1ea98f"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_texStorage3D_19979792a7a67f59": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texStorage3D_19979792a7a67f59"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_texSubImage2D_421e29fed0db07ab": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texSubImage2D_421e29fed0db07ab"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_texSubImage2D_f06e46b3b25ee691": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texSubImage2D_f06e46b3b25ee691"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_texSubImage3D_ebb9e6f80d19a411": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texSubImage3D_ebb9e6f80d19a411"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32);
/******/ 					},
/******/ 					"__wbg_texSubImage3D_591b8511a3c7593a": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texSubImage3D_591b8511a3c7593a"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32);
/******/ 					},
/******/ 					"__wbg_uniform2fv_a611afaf4a045f7e": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform2fv_a611afaf4a045f7e"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform2iv_b1b33c9425d5791b": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform2iv_b1b33c9425d5791b"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform3fv_740a7286bf6328ee": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform3fv_740a7286bf6328ee"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform3iv_df752fa54b2b8b7b": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform3iv_df752fa54b2b8b7b"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform4fv_737873ef0bcd5e6c": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4fv_737873ef0bcd5e6c"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform4iv_67eed4073c7e55c5": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4iv_67eed4073c7e55c5"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniformBlockBinding_50ced0c985f91a02": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformBlockBinding_50ced0c985f91a02"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix2fv_f4fc5e6214cc5549": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix2fv_f4fc5e6214cc5549"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix3fv_a02aa02ecb8e5f99": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix3fv_a02aa02ecb8e5f99"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix4fv_68d11b378757596e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix4fv_68d11b378757596e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribDivisor_2dc16945a591d4c6": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_vertexAttribDivisor_2dc16945a591d4c6"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribIPointer_167c7ed4319992e7": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_vertexAttribIPointer_167c7ed4319992e7"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_6a9afd67cc0ade73": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_activeTexture_6a9afd67cc0ade73"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_attachShader_90ad543fb1bccb18": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_attachShader_90ad543fb1bccb18"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_66e359418f5c82d7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindBuffer_66e359418f5c82d7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_5c01742edd5d843a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindFramebuffer_5c01742edd5d843a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindRenderbuffer_f66dee160b94e5ef": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindRenderbuffer_f66dee160b94e5ef"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_ae9620ea4a6ffb97": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindTexture_ae9620ea4a6ffb97"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendColor_50e203e2f58784cb": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendColor_50e203e2f58784cb"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_blendEquation_72746aedc87e3f72": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendEquation_72746aedc87e3f72"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_blendEquationSeparate_f0abe930082fff02": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendEquationSeparate_f0abe930082fff02"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendFunc_99b48b64bde98c6f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendFunc_99b48b64bde98c6f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendFuncSeparate_cecb7dfda39dc38d": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendFuncSeparate_cecb7dfda39dc38d"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_colorMask_12687df5490e9bc9": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_colorMask_12687df5490e9bc9"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_compileShader_822f38928f6f2a08": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compileShader_822f38928f6f2a08"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_copyTexSubImage2D_4c72e3ef713b65e6": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_copyTexSubImage2D_4c72e3ef713b65e6"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_a6cffb7f7d5b92a3": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createBuffer_a6cffb7f7d5b92a3"](p0i32);
/******/ 					},
/******/ 					"__wbg_createFramebuffer_d5f3985ce3652661": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createFramebuffer_d5f3985ce3652661"](p0i32);
/******/ 					},
/******/ 					"__wbg_createProgram_dc6b23d3caa1d86e": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createProgram_dc6b23d3caa1d86e"](p0i32);
/******/ 					},
/******/ 					"__wbg_createRenderbuffer_531167a301a60e27": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createRenderbuffer_531167a301a60e27"](p0i32);
/******/ 					},
/******/ 					"__wbg_createShader_46a66dce5a9e22d0": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createShader_46a66dce5a9e22d0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createTexture_269f67d411bdc4dc": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createTexture_269f67d411bdc4dc"](p0i32);
/******/ 					},
/******/ 					"__wbg_cullFace_d6b862a4ad70b414": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_cullFace_d6b862a4ad70b414"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteBuffer_12fd7d93834069ef": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteBuffer_12fd7d93834069ef"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteFramebuffer_d7551444a28f508e": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteFramebuffer_d7551444a28f508e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteProgram_ce56000628d7f1ce": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteProgram_ce56000628d7f1ce"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteRenderbuffer_58c540348fb8606d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteRenderbuffer_58c540348fb8606d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteShader_246e6e678f3eb957": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteShader_246e6e678f3eb957"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteTexture_68a539339fd87792": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteTexture_68a539339fd87792"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthFunc_1015c3364a49cd2f": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthFunc_1015c3364a49cd2f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthMask_55f538b7411e5023": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthMask_55f538b7411e5023"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthRange_c6ed3371d3b601f8": function(p0i32,p1f32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthRange_c6ed3371d3b601f8"](p0i32,p1f32,p2f32);
/******/ 					},
/******/ 					"__wbg_disable_1659dc1efb5fb934": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_disable_1659dc1efb5fb934"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_disableVertexAttribArray_6f3d27dd0ad6aabf": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_disableVertexAttribArray_6f3d27dd0ad6aabf"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawArrays_d587302f7a868d91": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawArrays_d587302f7a868d91"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_drawElements_241caa588795bcb1": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawElements_241caa588795bcb1"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_enable_4791414dce6f602a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_enable_4791414dce6f602a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_a1ffc091f3999354": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_enableVertexAttribArray_a1ffc091f3999354"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_framebufferRenderbuffer_963b305ac8cb6fd6": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_framebufferRenderbuffer_963b305ac8cb6fd6"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_framebufferTexture2D_4b810902dffa1ef3": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_framebufferTexture2D_4b810902dffa1ef3"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_frontFace_97d7f9493791771d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_frontFace_97d7f9493791771d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getActiveUniform_97472b76b9daa461": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getActiveUniform_97472b76b9daa461"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getExtension_e7912bce04869d40": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getExtension_e7912bce04869d40"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getParameter_4e2ccc745690476a": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getParameter_4e2ccc745690476a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getProgramInfoLog_1e37a3d1d090ec1c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getProgramInfoLog_1e37a3d1d090ec1c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getProgramParameter_acf4ae158143e2b2": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getProgramParameter_acf4ae158143e2b2"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderInfoLog_451545b963646762": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getShaderInfoLog_451545b963646762"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_6cd8c36fded266ea": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getShaderParameter_6cd8c36fded266ea"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getSupportedExtensions_b84494641d686623": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getSupportedExtensions_b84494641d686623"](p0i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_0da0c93f626244a2": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getUniformLocation_0da0c93f626244a2"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_c33885d9ea798810": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_linkProgram_c33885d9ea798810"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_pixelStorei_51c83dc5117bea35": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_pixelStorei_51c83dc5117bea35"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_polygonOffset_7af170d91752512c": function(p0i32,p1f32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_polygonOffset_7af170d91752512c"](p0i32,p1f32,p2f32);
/******/ 					},
/******/ 					"__wbg_renderbufferStorage_0b6269243d09a9f7": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_renderbufferStorage_0b6269243d09a9f7"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_scissor_b1b9e314ab6aac29": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_scissor_b1b9e314ab6aac29"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_5111981e7afb61fb": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_shaderSource_5111981e7afb61fb"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_stencilFuncSeparate_2939e543fa4caa77": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilFuncSeparate_2939e543fa4caa77"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_stencilMask_4eb0f989e4108b15": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilMask_4eb0f989e4108b15"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_stencilMaskSeparate_69e9937a9533f4ab": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilMaskSeparate_69e9937a9533f4ab"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_stencilOpSeparate_c57c8bbe863e9f57": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilOpSeparate_c57c8bbe863e9f57"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_21fd6b6b394882c9": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texParameteri_21fd6b6b394882c9"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform1f_ade6c204580582c8": function(p0i32,p1i32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform1f_ade6c204580582c8"](p0i32,p1i32,p2f32);
/******/ 					},
/******/ 					"__wbg_uniform1i_49986febd844f2c4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform1i_49986febd844f2c4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_uniform4f_d564461a6e4fdfe0": function(p0i32,p1i32,p2f32,p3f32,p4f32,p5f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4f_d564461a6e4fdfe0"](p0i32,p1i32,p2f32,p3f32,p4f32,p5f32);
/******/ 					},
/******/ 					"__wbg_useProgram_35a58ac1e0d9577b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_useProgram_35a58ac1e0d9577b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_3b06d737566f0745": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_vertexAttribPointer_3b06d737566f0745"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_viewport_319ab5302767fcc9": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_viewport_319ab5302767fcc9"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_acc97ff9f5d2c7b4": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_instanceof_Window_acc97ff9f5d2c7b4"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_3ead31dbcad65886": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_document_3ead31dbcad65886"](p0i32);
/******/ 					},
/******/ 					"__wbg_setInterval_b6f2e23785929613": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_setInterval_b6f2e23785929613"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_now_8172cd917e5eda6b": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_now_8172cd917e5eda6b"](p0i32);
/******/ 					},
/******/ 					"__wbg_drawBuffersWEBGL_482a093ae5a4ad55": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawBuffersWEBGL_482a093ae5a4ad55"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_97761617af6ea089": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_97761617af6ea089"](p0i32);
/******/ 					},
/******/ 					"__wbg_setwidth_afb418d3fbf71ba7": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_setwidth_afb418d3fbf71ba7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setheight_3eb8729b59493242": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_setheight_3eb8729b59493242"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getContext_a6ea7a8e317f182a": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getContext_a6ea7a8e317f182a"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_code_06787cd3c7a60600": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_code_06787cd3c7a60600"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_preventDefault_3209279b490de583": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_preventDefault_3209279b490de583"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_3a708b83e4f034d7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getElementById_3a708b83e4f034d7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_querySelector_3628dc2c3319e7e0": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_querySelector_3628dc2c3319e7e0"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_attributes_a4140d6795dd5707": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_attributes_a4140d6795dd5707"](p0i32);
/******/ 					},
/******/ 					"__wbg_setAttribute_d8436c14a59ab1af": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_setAttribute_d8436c14a59ab1af"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_style_e9380748cee29f13": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_style_e9380748cee29f13"](p0i32);
/******/ 					},
/******/ 					"__wbg_bufferData_d6fac0d761e08fec": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferData_d6fac0d761e08fec"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bufferData_a33528a74dd300f4": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferData_a33528a74dd300f4"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bufferSubData_a116fea11850b38f": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bufferSubData_a116fea11850b38f"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_compressedTexSubImage2D_30943b654d04ee44": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compressedTexSubImage2D_30943b654d04ee44"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32);
/******/ 					},
/******/ 					"__wbg_readPixels_db685489e1779d63": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_readPixels_db685489e1779d63"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32);
/******/ 					},
/******/ 					"__wbg_texSubImage2D_cb339dd200dd1179": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texSubImage2D_cb339dd200dd1179"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_uniform2fv_3aad4d306a1cb8af": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform2fv_3aad4d306a1cb8af"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform2iv_8c390eac30cb1de3": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform2iv_8c390eac30cb1de3"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform3fv_d1ef35c158c348e7": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform3fv_d1ef35c158c348e7"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform3iv_76acc51e8e6fe1a4": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform3iv_76acc51e8e6fe1a4"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform4fv_a513dc4d02f192d3": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4fv_a513dc4d02f192d3"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform4iv_19aa13960dc767c2": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4iv_19aa13960dc767c2"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix2fv_4173a282fcaa5508": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix2fv_4173a282fcaa5508"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix3fv_2b7de3010c8ed627": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix3fv_2b7de3010c8ed627"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix4fv_f16e4a5553357886": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniformMatrix4fv_f16e4a5553357886"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_02b7c73c76c2c06b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_activeTexture_02b7c73c76c2c06b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_attachShader_f4d51147351a1906": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_attachShader_f4d51147351a1906"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_8b5135aa633680f5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindBuffer_8b5135aa633680f5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_080d0b0cf22e1645": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindFramebuffer_080d0b0cf22e1645"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindRenderbuffer_6da549f066c1b8a5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindRenderbuffer_6da549f066c1b8a5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_6f1dec563e82e818": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindTexture_6f1dec563e82e818"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendColor_3bea829c60b1f6f2": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendColor_3bea829c60b1f6f2"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_blendEquation_5d5abe2ee10109a9": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendEquation_5d5abe2ee10109a9"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_blendEquationSeparate_fa6aebc5cd0c5285": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendEquationSeparate_fa6aebc5cd0c5285"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendFunc_49ea28240d4c1084": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendFunc_49ea28240d4c1084"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendFuncSeparate_9fef8acb74d50df5": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_blendFuncSeparate_9fef8acb74d50df5"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_colorMask_bc13c97d0db65962": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_colorMask_bc13c97d0db65962"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_compileShader_22b038faa1f49857": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_compileShader_22b038faa1f49857"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_copyTexSubImage2D_e815f93a9ef52dd2": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_copyTexSubImage2D_e815f93a9ef52dd2"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_6e747d928c9ba46d": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createBuffer_6e747d928c9ba46d"](p0i32);
/******/ 					},
/******/ 					"__wbg_createFramebuffer_9b5b0507480146cd": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createFramebuffer_9b5b0507480146cd"](p0i32);
/******/ 					},
/******/ 					"__wbg_createProgram_1c5f8dffd1066e71": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createProgram_1c5f8dffd1066e71"](p0i32);
/******/ 					},
/******/ 					"__wbg_createRenderbuffer_69c2f0554298bf89": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createRenderbuffer_69c2f0554298bf89"](p0i32);
/******/ 					},
/******/ 					"__wbg_createShader_4017d9fbc36659af": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createShader_4017d9fbc36659af"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createTexture_4ce49e8a8c655124": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createTexture_4ce49e8a8c655124"](p0i32);
/******/ 					},
/******/ 					"__wbg_cullFace_aa9f8eea262690c0": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_cullFace_aa9f8eea262690c0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteBuffer_6fd9bca7f8a6d9de": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteBuffer_6fd9bca7f8a6d9de"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteFramebuffer_2617e39d2c39b4da": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteFramebuffer_2617e39d2c39b4da"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteProgram_e8636e3cb5a18a59": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteProgram_e8636e3cb5a18a59"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteRenderbuffer_e5b3450b8b57b395": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteRenderbuffer_e5b3450b8b57b395"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteShader_89369612f61ec145": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteShader_89369612f61ec145"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteTexture_5c40169772519141": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteTexture_5c40169772519141"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthFunc_2ac2c797a8220f09": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthFunc_2ac2c797a8220f09"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthMask_88ab181c23c32dcd": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthMask_88ab181c23c32dcd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_depthRange_5dccc27b5cdd74b3": function(p0i32,p1f32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_depthRange_5dccc27b5cdd74b3"](p0i32,p1f32,p2f32);
/******/ 					},
/******/ 					"__wbg_disable_6835d16c2cd3fa26": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_disable_6835d16c2cd3fa26"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_disableVertexAttribArray_ab474d273ff59265": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_disableVertexAttribArray_ab474d273ff59265"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawArrays_c0dcb4151e0bf007": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawArrays_c0dcb4151e0bf007"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_drawElements_e09dbef58c8f099a": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawElements_e09dbef58c8f099a"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_enable_fc393941ac400f72": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_enable_fc393941ac400f72"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_3d21f4936ad4a378": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_enableVertexAttribArray_3d21f4936ad4a378"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_framebufferRenderbuffer_6b8dd5a111d341e6": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_framebufferRenderbuffer_6b8dd5a111d341e6"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_framebufferTexture2D_499d1c21458d0113": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_framebufferTexture2D_499d1c21458d0113"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_frontFace_5fd354be6327d46b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_frontFace_5fd354be6327d46b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getActiveUniform_fd021da851153e8c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getActiveUniform_fd021da851153e8c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getParameter_585a5b83c595ada8": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getParameter_585a5b83c595ada8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getProgramInfoLog_e47d5073d57fb18d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getProgramInfoLog_e47d5073d57fb18d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getProgramParameter_eaf768a9b399b7cf": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getProgramParameter_eaf768a9b399b7cf"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderInfoLog_ec7e5b959e47645b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getShaderInfoLog_ec7e5b959e47645b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_42a35b974329561c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getShaderParameter_42a35b974329561c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_8e9cc276a231ddcd": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getUniformLocation_8e9cc276a231ddcd"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_25cda5f9318ea316": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_linkProgram_25cda5f9318ea316"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_pixelStorei_bee1e2da4cb1115b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_pixelStorei_bee1e2da4cb1115b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_polygonOffset_4cba459d8eacb66d": function(p0i32,p1f32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_polygonOffset_4cba459d8eacb66d"](p0i32,p1f32,p2f32);
/******/ 					},
/******/ 					"__wbg_renderbufferStorage_4ceec9b17dbd1e76": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_renderbufferStorage_4ceec9b17dbd1e76"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_scissor_4b89b60091ee8f0e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_scissor_4b89b60091ee8f0e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_a0001b8eab5d44f4": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_shaderSource_a0001b8eab5d44f4"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_stencilFuncSeparate_1f0226d5d3acaf47": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilFuncSeparate_1f0226d5d3acaf47"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_stencilMask_00541859199befd2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilMask_00541859199befd2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_stencilMaskSeparate_5e7b9b536eac0c5d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilMaskSeparate_5e7b9b536eac0c5d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_stencilOpSeparate_153523493abc8ec8": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stencilOpSeparate_153523493abc8ec8"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_1b210b807f1ea723": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_texParameteri_1b210b807f1ea723"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform1f_f60e1072e28b8c49": function(p0i32,p1i32,p2f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform1f_f60e1072e28b8c49"](p0i32,p1i32,p2f32);
/******/ 					},
/******/ 					"__wbg_uniform1i_50124a48de1da66b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform1i_50124a48de1da66b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_uniform4f_5b62a0acebac4494": function(p0i32,p1i32,p2f32,p3f32,p4f32,p5f32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_uniform4f_5b62a0acebac4494"](p0i32,p1i32,p2f32,p3f32,p4f32,p5f32);
/******/ 					},
/******/ 					"__wbg_useProgram_156511a425feb519": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_useProgram_156511a425feb519"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_63d2aef49627302b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_vertexAttribPointer_63d2aef49627302b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_viewport_a93f3881c4202d5e": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_viewport_a93f3881c4202d5e"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_value_5ad7478d7216c125": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_value_5ad7478d7216c125"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setProperty_e489dfd8c0a6bffc": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_setProperty_e489dfd8c0a6bffc"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_getNamedItem_bcfc3b5818f403a4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getNamedItem_bcfc3b5818f403a4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_size_878ba1bf0c2ec606": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_size_878ba1bf0c2ec606"](p0i32);
/******/ 					},
/******/ 					"__wbg_type_ca7819eaadc2049f": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_type_ca7819eaadc2049f"](p0i32);
/******/ 					},
/******/ 					"__wbg_name_2473476082bed625": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_name_2473476082bed625"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawArraysInstancedANGLE_89a45d6f51cd0483": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawArraysInstancedANGLE_89a45d6f51cd0483"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_drawElementsInstancedANGLE_6ac21f9a1ebe5f6b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_drawElementsInstancedANGLE_6ac21f9a1ebe5f6b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribDivisorANGLE_d5931335aaf0c735": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_vertexAttribDivisorANGLE_d5931335aaf0c735"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_addEventListener_cbe4c6f619b032f3": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_addEventListener_cbe4c6f619b032f3"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bindVertexArrayOES_84540c072ea96b75": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_bindVertexArrayOES_84540c072ea96b75"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createVertexArrayOES_00a5c523e5b17eff": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_createVertexArrayOES_00a5c523e5b17eff"](p0i32);
/******/ 					},
/******/ 					"__wbg_deleteVertexArrayOES_98b83132b3d85825": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_deleteVertexArrayOES_98b83132b3d85825"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_get_57245cc7d7c7619d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_get_57245cc7d7c7619d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_length_6e3bbe7c8bd4dbd8": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_length_6e3bbe7c8bd4dbd8"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_1d9a920c6bfc44a8": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_new_1d9a920c6bfc44a8"]();
/******/ 					},
/******/ 					"__wbg_newnoargs_b5b063fc6c2f0376": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newnoargs_b5b063fc6c2f0376"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_get_765201544a2b6869": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_get_765201544a2b6869"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_97ae9d8645dc388b": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_call_97ae9d8645dc388b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_0b9bfdd97583284e": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_new_0b9bfdd97583284e"]();
/******/ 					},
/******/ 					"__wbg_self_6d479506f72c6a71": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_self_6d479506f72c6a71"]();
/******/ 					},
/******/ 					"__wbg_window_f2557cc78490aceb": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_window_f2557cc78490aceb"]();
/******/ 					},
/******/ 					"__wbg_globalThis_7f206bda628d5286": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_globalThis_7f206bda628d5286"]();
/******/ 					},
/******/ 					"__wbg_global_ba75c50d1cf384f4": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_global_ba75c50d1cf384f4"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_of_d79bf3cec607f7a4": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_of_d79bf3cec607f7a4"](p0i32);
/******/ 					},
/******/ 					"__wbg_push_740e4b286702d964": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_push_740e4b286702d964"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_is_40a66842732708e7": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_is_40a66842732708e7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_resolve_99fe17964f31ffc0": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_resolve_99fe17964f31ffc0"](p0i32);
/******/ 					},
/******/ 					"__wbg_then_11f7a54d67b4bfad": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_then_11f7a54d67b4bfad"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_buffer_3f3d764d4747d564": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_buffer_3f3d764d4747d564"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_890b478c8d7226ff": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_890b478c8d7226ff"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_698c5100ae9c3365": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_698c5100ae9c3365"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_7be13f49af2b2012": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_7be13f49af2b2012"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_d9aa266703cb98be": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_d9aa266703cb98be"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_length_9e1ae1900cb0fbd5": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_length_9e1ae1900cb0fbd5"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_5540e144e9b8b907": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_5540e144e9b8b907"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_9cc9adccd861aa26": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_9cc9adccd861aa26"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_be22e5fcf4f69ab4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_newwithbyteoffsetandlength_be22e5fcf4f69ab4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Uint8Array_971eeda69eb75003": function(p0i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_instanceof_Uint8Array_971eeda69eb75003"](p0i32);
/******/ 					},
/******/ 					"__wbg_getindex_ed9af38a6f2f9635": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_getindex_ed9af38a6f2f9635"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_set_bf3f89b92d5a34bf": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_set_bf3f89b92d5a34bf"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_floor_182f4f67bb2a06bd": function(p0f64) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_floor_182f4f67bb2a06bd"](p0f64);
/******/ 					},
/******/ 					"__wbg_random_656f2ae924b2540e": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_random_656f2ae924b2540e"]();
/******/ 					},
/******/ 					"__wbg_new_abda76e883ba8a5f": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_new_abda76e883ba8a5f"]();
/******/ 					},
/******/ 					"__wbg_stack_658279fe44541cf6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_stack_658279fe44541cf6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_f851667af71bcfc6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbg_error_f851667af71bcfc6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper408": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_closure_wrapper408"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper410": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_closure_wrapper410"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper570": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/libnoentiendo_bg.js"].exports["__wbindgen_closure_wrapper570"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/libnoentiendo_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/libnoentiendo_bg.wasm":"dfca642383be14007837"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });