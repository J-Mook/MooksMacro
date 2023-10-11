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
use winapi::um::winuser::{GetCursorPos, WindowFromPoint, GetWindowRect, GetAsyncKeyState, VK_F4, VK_F2, VK_CONTROL};
use winapi::shared::windef::{POINT, RECT};

static mut DEBUGMODE : bool = false;

macro_rules! debug_println {
    ($($arg:tt)*) => {
        if get_debug_mode() {
            println!($($arg)*);
        }
    };
}

fn get_debug_mode() -> bool {
    unsafe {
        DEBUGMODE
    }
}
fn main() {

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
    
    loop{
        mooks_macro();
    }
}

fn mooks_macro() {
    
    println!("학습 창을 클릭하고 F2를 누르세요...");
    println!("");
    loop{
        unsafe {
            let ctrl_pressed = GetAsyncKeyState(VK_CONTROL) & 0x8000u16 as i16 != 0;
            let d_pressed = GetAsyncKeyState(0x44) & 0x8000u16 as i16 != 0;  // 'D' 키의 가상 키 코드는 0x44입니다.
            if ctrl_pressed && d_pressed {
                println!("DEBUG MODE ON");
                DEBUGMODE = true;
            }

            let f2_pressed = GetAsyncKeyState(VK_F2) & 0x8000u16 as i16 != 0;
            if f2_pressed {
                println!("(학습 창의 위치를 기억합니다. 매크로가 실행되는 동안 학습 창을 움직이지 마세요.)");
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }

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
            debug_println!("Selected Winodw Coord: ({}, {})", rect.left, rect.top);
        } else {
            debug_println!("Failed Window Coord. Set Coord Default");
            rect.left = 468; 
            rect.top = 66;
        }
    }
    
    println!("정지하려면 F4를 누르세요...");
    let mut enigo: Enigo = Enigo::new();
    loop {
        let mut stop_sign = false;

        click2seq(&mut enigo, &rect, &mut stop_sign);
        
        click2img(&mut enigo, &mut stop_sign);
        
        if stop_sign || milli_wait_stop(10000){
            break;
        }
    }
}

fn milli_wait_stop(timeout :u64) -> bool{

    let mut cnt = 0;
    while cnt < (timeout / 100) as i32 {
        unsafe {
            let f4_pressed = GetAsyncKeyState(VK_F4) & 0x8000u16 as i16 != 0;
            if f4_pressed {
                return true;
            }
        }
        cnt += 1;
        thread::sleep(Duration::from_millis(100));
    }
    return false;
}

fn click2seq(enigo: &mut Enigo, rect: &RECT, stop_sign: &mut bool){

    if *stop_sign {
        return;
    }
    // sequence.seq 파일 읽어서 클릭
    if let Ok(file) = File::open("sequence.seq") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let coords: Vec<&str> = line.trim().split('\t').collect();
                if coords.len() == 2 {
                    if let (Ok(x), Ok(y)) = (coords[0].parse::<i32>(), coords[1].parse::<i32>()) {
                        debug_println!("Sequence Coord ({}, {})", x, y);
                        enigo.mouse_move_to(rect.left + x, rect.top + y);
                        enigo.mouse_click(enigo::MouseButton::Left);
                    }
                }
            }
            if milli_wait_stop(500){
                *stop_sign = true;
                return;
            }
        }
    }
}
fn click2img(enigo: &mut Enigo, stop_sign: &mut bool){

    if *stop_sign {
        return;
    }
    //image data 이미지 찾기
    let screens = Screen::all().unwrap();
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let mut image_data_dir = PathBuf::from(exe_path.parent().expect("Failed to get parent directory"));
    image_data_dir.push("image_data");
    
    for entry in WalkDir::new(image_data_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        debug_println!("{}", &path.display());
        for screen in &screens {
            let screenshot_buffer = screen.capture().unwrap();
            let screenshot = rgba2dynimg(screenshot_buffer);
            if path.extension().map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "JPG" || ext == "PNG") {
                let image_data = image::open(&path).expect("Failed to open image");
                if let Some((x, y)) = find_image(&screenshot, &image_data) {
                    let (img_width, img_height) = image_data.dimensions();
                    let click_x = x as i32 + (img_width / 2) as i32 + screen.display_info.x as i32;
                    let click_y = y as i32 + (img_height / 2) as i32 + screen.display_info.y as i32;
                    debug_println!("Found Image Center ({}, {})", click_x, click_y);
                    enigo.mouse_move_to(click_x, click_y);
                    enigo.mouse_click(enigo::MouseButton::Left);
                    if milli_wait_stop(500){
                        *stop_sign = true;
                        return;
                    }
                }
            }
        }
    }
}

fn rgba2dynimg(img_buffer: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>>) -> image::DynamicImage {
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
