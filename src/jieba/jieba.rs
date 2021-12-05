use jieba_rs::Jieba;
use pinyin::{Pinyin, to_pinyin_vec, ToPinyin, ToPinyinMulti};

pub fn name_to_pinyin(name: &str) -> Option<String> {
    let pinyinwords = name.to_pinyin();

    let pinyin_string = pinyinwords.map(move |word| {
                            match word {
                                None => {
                                    "".to_string()
                                }
                                Some(w) => {
                                    w.plain().to_string()
                                }
                            }
                        }).collect::<String>();

    if pinyin_string.is_empty() {
        None
    } else {
        Some(pinyin_string)
    }


}

pub fn tokenize(sentence: &str) -> Vec<String> {
    let jieba = Jieba::new();

    jieba.cut(sentence, false).into_iter().map(|w| w.to_string()).collect()
}

pub fn transliterate_tokenized(sentence: &str) -> String {
    let tokenized_sentence = tokenize(sentence);

    tokenized_sentence.into_iter().map(move|token| {
        let pinyin = name_to_pinyin(&token);

        if pinyin.is_some() {
            pinyin.unwrap()
        } else {
            token
        }

    }).collect::<Vec<String>>().join(" ")
}

pub fn transliterate_full(sentence: &str) -> String {

        let pinyin = name_to_pinyin(sentence);

        if pinyin.is_some() {
            pinyin.unwrap()
        } else {
            sentence.to_string()
        }

}

#[cfg(test)]
mod test_pinyin {
    use crate::jieba::jieba::{name_to_pinyin, tokenize, transliterate_tokenized};

    #[test]
    fn to_pinyin() {
        let result = name_to_pinyin("中国").unwrap();
        assert_eq!(result, "zhongguo")
    }

    #[test]
    fn to_pinyin_english() {
        let result = name_to_pinyin("China");
        assert_eq!(result, None)
    }

    #[test]
    fn test_segmentation() {
        let result = tokenize("中国China人民纪念碑People");

        assert_eq!(result.iter().count(), 5);
        assert_eq!(result.last().unwrap(), "People");
    }

    #[test]
    fn test_transliterate() {
        let result = transliterate_tokenized("中国China人民的纪念碑People");
        assert_eq!(result, "zhongguo China renmin de jinianbei People")
    }


    #[test]
    fn test_transliterate_name() {
        let result = transliterate_tokenized("钱学森");
        assert_eq!(result, "qian xue sen")
    }
}
