use id3::{Tag, frame::Picture, frame::PictureType};

pub fn get_music_cover(file_path: String) {
    // 打开 MP3 文件的 ID3 标签
    let tag = Tag::read_from_path(file_path).expect("Failed to read ID3 tag");

    // 遍历并打印所有图片类型
    for picture in tag.pictures() {
        println!("Found picture with type: {:?}", picture.picture_type);
    }

    // 查找封面图片（包括其他类型）
    let cover_image = tag.pictures()
        .find(|pic| pic.picture_type == PictureType::CoverFront || pic.picture_type == PictureType::Other);

    match cover_image {
        Some(picture) => {
            // 保存封面图片到文件
            let image_path = "cover.jpg";
            std::fs::write(image_path, &picture.data).expect("Failed to write cover image");
            println!("Cover image saved to {}", image_path);
        }
        None => println!("No cover image found in the MP3 file."),
    }
}