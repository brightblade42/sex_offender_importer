Vim�UnDo� 4 0a��~��K�� ��ef��$�pCad��   d       },                             bO��    _�                            ����                                                                                                                                                                                                                                                                                                                                                             bOCe     �         d      .use rusqlite::{Connection, NO_PARAMS, params};5��              	           �       	               5�_�                           ����                                                                                                                                                                                                                                                                                                                                                             bOCf    �         d      %use rusqlite::{Connection, , params};5��                         �                      5�_�                    6        ����                                                                                                                                                                                                                                                                                                                                                             bOCw    �   \   ^          h        conn.execute( &format!("DELETE FROM {} where state='{}'", table_name, &self.state), NO_PARAMS, )�   L   N          8        conn.execute("COMMIT TRANSACTION;", NO_PARAMS)?;�   5   7   d      7        conn.execute("BEGIN TRANSACTION;", NO_PARAMS)?;5��    5   +       	          �      	              �    L   ,       	          ~	      	              �    \   \       	          �      	              5�_�                           ����                                                                                                                                                                                                                                                                                                                                                             bO��     �         d      use crate::{5��                         N                      5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             bO��     �         d      };5��                          �                      5�_�                            ����                                                                                                                                                                                                                                                                                                                                                             bO��    �   
      d          },5��    
                     �                      5��