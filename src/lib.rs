#![feature(impl_trait_in_assoc_type)]

use volo::*;
use std::{collections::HashMap, sync::Mutex};
use pilota::FastStr;
use volo_gen::volo::example::ItemService;
pub struct S{
	pub map: Mutex<HashMap<String, String>>,
}

use 
#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get_item(&self, 
                _req: volo_gen::volo::example::GetItemRequest
            ) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>
            {
                match _req.cmd {
                    Commands::Get => {
                        if let Some(arg) = _req.args {
                        if arg.len() == 1{
                            if let Some(value) = self.map.lock().unwrap().get(&arg[0].to_string()) {
                                OK(GetItemResponse {
                                    ok: true,
                                    msg: Some(FastStr::from(value.to_string())),
                                }) 
                                                    
                            } else {
                                OK(GetItemResponse {
                                    ok: false,
                                    msg: Some(FastStr::from("not found"))
                                })
                            }
                        } else {
                            OK(GetItemResponse {
                                ok: false,
                                msg: Some(FastStr::from(format!("too many args: {}", arg.len())))
                            })
                        }
                    } else {
                        OK(GetItemResponse {
                            ok: false,
                            msg: Some(FastStr::from("no arg"))
                        })
                    }
                    }
                    Commands::Set => {
                        if let Some(arg) = _req.args {
                        if arg.len() == 2 {
                            let(key, value) = (&arg[0], &arg[1]);
                            if self.map.lock().unwrap().insert(key.to_string(), value.to_string()).is_some() {
                                OK(GetItemResponse {
                                    ok: true,
                                    msg: Some(FastStr::from("ok, the value has been updated"))
                                })
                            } else {
                                OK(GetItemResponse {
                                    ok: true,
                                    msg: Some(FastStr::from("ok, the value has been added"))
                                })
                            }
                            
                        } else {
                            OK(GetItemResponse {
                                ok: false,
                                msg: Some(FastStr::from(format!("not 2 args but: {}", arg.len())))
                            })
                        }
    
                    } else {
                        OK(GetItemResponse {
                            ok: false,
                            msg: Some(FastStr::from("no arg"))
                        })
                    }
                    
                }
                Commands::Del => {
                    if let Some(arg) = _req.args {
                        if arg.len() == 1 {
                            let mut cnt = 0;
                            for key in arg {
                                cnt += self.map.try_lock().unwrap().remove(&key.to_string()).is_some();
                            }
                            OK(GetItemResponse {
                                ok: true,
                                msg: Some(FastStr::from(format!("ok, the value has been deleted: {}", cnt)))
                            })
                        } else {
                            OK(GetItemResponse {
                                ok: false,
                                msg: Some(FastStr::from(format!("not 1 args but : {}", arg.len())))
                            })
                        }
                    } else {
                        OK(GetItemResponse {
                            ok: false,
                            msg: Some(FastStr::from("no arg"))
                        })

                }

                }
                Commands::Ping => {
                    if let Some(arg) = _req.args {
                        if arg.len() > 0 {
                            OK(GetItemResponse {
                                ok: true,
                                msg: Some(FastStr::from(arg.join(" ")))
                            })
                        } else {
                            OK(GetItemResponse {
                                ok: true,
                                msg: Some(FastStr::from("pong"))
                            })
                        }
                    } else {
                        OK(GetItemResponse {
                            ok: true,
                            msg: Some(FastStr::from("pong"))
                        })
                    }

                }
                _ => {
                    Ok(GetItemResponse {
                        ok: false,
                        msg: Some(FastStr::from("unknown command"))
                    })
                }
            }
        }
    }

    pub struct LogLayer;

    impl<S> volo::Layer<S> for LogLayer {
        type Service = LogService<S>;
    
        fn layer(self, inner: S) -> Self::Service {
            LogService(inner)
        }
    }

    #[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
    
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {

        let info = format!("{:?}", req);
		if info.contains("shabi") {
            Err(anyhow!("No dirty word, please!").into())
			// Err(S::Error::from(Error::msg("Genshin is not allowed")))
		} else {
			self.0.call(cx, req).await
		}
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}