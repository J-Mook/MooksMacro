extern crate walkdir;
extern crate image;
extern crate screenshots;
extern crate enigo;

use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;
use image::GenericImageView;
use enigo::{Enigo, MouseControllable};
use screenshots::Screen;
use std::{thread, time::Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};
use winapi::um::winuser::{GetCursorPos, WindowFromPoint, GetWindowRect, GetAsyncKeyState, VK_F4, VK_F2};
use winapi::shared::windef::{POINT, RECT};

fn main() {
    loop{
        find_and_click_images();
    }
}

fn find_and_click_images() {
    println!(r"
    __       __                      __        __                                                                    
    /  \     /  |                    /  |      /  |                                                                   
    $$  \   /$$ |  ______    ______  $$ |   __ $$/_______        _____  ____    ______    _______   ______    ______  
    $$$  \ /$$$ | /      \  /      \ $$ |  /  |$//       |      /     \/    \  /      \  /       | /      \  /      \ 
    $$$$  /$$$$ |/$$$$$$  |/$$$$$$  |$$ |_/$$/  /$$$$$$$/       $$$$$$ $$$$  | $$$$$$  |/$$$$$$$/ /$$$$$$  |/$$$$$$  |
    $$ $$ $$/$$ |$$ |  $$ |$$ |  $$ |$$   $$<   $$      \       $$ | $$ | $$ | /    $$ |$$ |      $$ |  $$/ $$ |  $$ |
    $$ |$$$/ $$ |$$ \__$$ |$$ \__$$ |$$$$$$  \   $$$$$$  |      $$ | $$ | $$ |/$$$$$$$ |$$ \_____ $$ |      $$ \__$$ |
    $$ | $/  $$ |$$    $$/ $$    $$/ $$ | $$  | /     $$/       $$ | $$ | $$ |$$    $$ |$$       |$$ |      $$    $$/ 
    $$/      $$/  $$$$$$/   $$$$$$/  $$/   $$/  $$$$$$$/        $$/  $$/  $$/  $$$$$$$/  $$$$$$$/ $$/        $$$$$$/  
    ");
    println!("
                                          ##    ###                        ######                       ##       ##
                                         ##      ##                         ##  ##                      ##        ##
                                        ##       ##      ##  ##             ##  ##  ##  ##    #####    #####       ##
                                        ##       #####   ##  ##             #####   ##  ##   ##         ##         ##
                                        ##       ##  ##  ##  ##             ## ##   ##  ##    #####     ##         ##
                                         ##      ##  ##   #####             ##  ##  ##  ##        ##    ## ##     ##
                                          ##    ######       ##            #### ##   ######  ######      ###     ##
                                                         #####
    ");
    println!("\n======================================================================================================================\n");
    
    let mut stop_sign = false;
    println!("학습 창을 클릭하고 F2를 누르세요...");
    loop{
        unsafe {
            let f2_pressed = GetAsyncKeyState(VK_F2) & 0x8000u16 as i16 != 0;
            if f2_pressed {
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    // thread::sleep(Duration::from_secs(10));
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    
    let mut pt = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut pt);
        let hwnd = WindowFromPoint(pt);
        
        if GetWindowRect(hwnd, &mut rect) != 0 {
            println!("Selected Winodw Coord: ({}, {})", rect.left, rect.top);
        } else {
            println!("Failed Window Coord. Set Coord Default");
            rect.left = 468; 
            rect.top = 66;
        }
    }
    
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // loop {
        //     let (x, y) = enigo.mouse_location();
        //     println!("Current mouse position: ({}, {})", x, y);
        //     std::thread::sleep(std::time::Duration::from_secs(1));
        // }
        
    let mut enigo = Enigo::new();
    loop {
        // sequence.seq 파일 읽어서 클릭
        if let Ok(file) = File::open("sequence.seq") {
            let reader = BufReader::new(file);
            for line
             in reader.lines() {
                if let Ok(line) = line {
                    let coords: Vec<&str> = line.trim().split('\t').collect();
                    if coords.len() == 2 {
                        if let (Ok(x), Ok(y)) = (coords[0].parse::<i32>(), coords[1].parse::<i32>()) {
                            println!("Sequence Coord ({}, {})", x, y);
                            enigo.mouse_move_to(rect.left + x, rect.top + y);
                            enigo.mouse_click(enigo::MouseButton::Left);
                            thread::sleep(Duration::from_millis(500));
                        }
                    }
                }
            }
        }

        //image data 이미지 찾기
        let screens = Screen::all().unwrap();
        
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        let mut image_data_dir = PathBuf::from(exe_path.parent().expect("Failed to get parent directory"));
        image_data_dir.push("image_data");
        
        for entry in WalkDir::new(image_data_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            println!("{}", &path.display());
            for screen in &screens {
                let screenshot_buffer = screen.capture().unwrap();
                let screenshot = convert_to_dynamic_image(screenshot_buffer);
                if path.extension().map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "JPG" || ext == "PNG") {
                    let image_data = image::open(&path).expect("Failed to open image");
                    if let Some((x, y)) = find_image(&screenshot, &image_data) {
                        let (img_width, img_height) = image_data.dimensions();
                        let click_x = x as i32 + (img_width / 2) as i32 + screen.display_info.x as i32;
                        let click_y = y as i32 + (img_height / 2) as i32 + screen.display_info.y as i32;
                        println!("Found Image Center ({}, {})", click_x, click_y);
                        enigo.mouse_move_to(click_x, click_y);
                        enigo.mouse_click(enigo::MouseButton::Left);
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        }
        
        let mut cnt = 0;
        println!("정지하려면 10초안에 F4를 누르세요...");
        loop{
            unsafe {
                let f4_pressed = GetAsyncKeyState(VK_F4) & 0x8000u16 as i16 != 0;
                if f4_pressed {
                    stop_sign = true;
                    break;
                }
                if cnt > 100 {
                    println!("늦었습니다.");
                    break;
                }
            }
            cnt += 1;
            thread::sleep(Duration::from_millis(100));
        }
        
        if stop_sign {
            break;
        }
    }
}

fn convert_to_dynamic_image(img_buffer: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>>) -> image::DynamicImage {
    let (width, height) = img_buffer.dimensions();
    let raw_data = img_buffer.into_raw();
    let img_buffer = image::ImageBuffer::from_raw(width, height, raw_data).unwrap();
    image::DynamicImage::ImageRgba8(img_buffer)
}

fn find_image(screenshot: &image::DynamicImage, img: &image::DynamicImage) -> Option<(u32, u32)> {
    let (width, height) = img.dimensions();
    for y in 0..(screenshot.height() - height) {
        for x in 0..(screenshot.width() - width) {
            let mut matches = true;
            for iy in 0..height {
                for ix in 0..width {
                    if screenshot.get_pixel(x + ix, y + iy) != img.get_pixel(ix, iy) {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    break;
                }
            }
            if matches {
                return Some((x, y));
            }
        }
    }
    None
}


// extern crate winapi;

// use winapi::um::winuser::{SetWindowsHookExW, CallNextHookEx, GetMessageW, UnhookWindowsHookEx, WH_MOUSE_LL};
// use winapi::um::winuser::{WM_LBUTTONDOWN, MSLLHOOKSTRUCT};
// use winapi::shared::minwindef::LPARAM;

// unsafe extern "system" fn hook_proc(n_code: i32, w_param: usize, l_param: LPARAM) -> isize {
//     if w_param as u32 == WM_LBUTTONDOWN {
//         let mouse_struct = *(l_param as *mut MSLLHOOKSTRUCT);
//         println!("Clicked at: ({}, {})", mouse_struct.pt.x, mouse_struct.pt.y);
//     }

//     CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
// }

// fn main() {
//     unsafe {
//         let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc as _), std::ptr::null_mut(), 0);
//         if hook.is_null() {
//             println!("Failed to set hook");
//             return;
//         }

//         let mut msg = std::mem::zeroed();
//         while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {}

//         UnhookWindowsHookEx(hook);
//     }
// }

