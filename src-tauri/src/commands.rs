mod slop1;
mod slop2;

use slop1::SlopStruct1;
use slop2::SlopStruct2;

pub struct MyState {}

// 1 command returning SlopStruct1
#[tauri::command]
pub async fn command_1(_: tauri::State<'_, MyState>) -> Result<SlopStruct1, ()> {
    Ok(SlopStruct1::new(0).await)
}

// 299 commands returning SlopStruct2
seq_macro::seq!(N in 2..=300 {
    #[tauri::command]
    pub async fn command_~N(_: tauri::State<'_, MyState>) -> Result<SlopStruct2, ()> {
        Ok(SlopStruct2::new(0).await)
    }
});
