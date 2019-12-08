// Copyright 2018 The HuggingFace Inc. team.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


pub mod preprocessing;
pub mod modeling;

pub use preprocessing::vocab::{base_vocab::BaseVocab, bert_vocab::BertVocab};
pub use preprocessing::tokenizer::bert_tokenizer;
use pyo3::prelude::*;
use crate::preprocessing::tokenizer::bert_tokenizer::BertTokenizer;
use crate::preprocessing::tokenizer::base_tokenizer::{Tokenizer, TruncationStrategy, TokenizedInput};
use pyo3::exceptions;

#[macro_use]
extern crate lazy_static;

#[pyclass(module = "rust_transformers")]
struct PyBertTokenizer {
    tokenizer: BertTokenizer,
}

#[pymethods]
impl PyBertTokenizer {
    #[new]
    fn new(obj: &PyRawObject, path: String) {
        obj.init(PyBertTokenizer {
            tokenizer: BertTokenizer::from_file(&path),
        });
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<String>> {
        Ok(self.tokenizer.tokenize(&text))
    }

    fn tokenize_list(&self, text_list: Vec<&str>) -> PyResult<Vec<Vec<String>>> {
        Ok(self.tokenizer.tokenize_list(text_list))
    }

    fn encode(&self, text: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer.encode(&text, None, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_pair(&self, text_a: &str, text_b: &str, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<TokenizedInput> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer.encode(&text_a, Some(&text_b), max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_list(&self, text_list: Vec<&str>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer.encode_list(text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }

    fn encode_pair_list(&self, text_list: Vec<(&str, &str)>, max_len: usize, truncation_strategy: &str, stride: usize) -> PyResult<Vec<TokenizedInput>> {
        let truncation_strategy = match truncation_strategy {
            "longest_first" => Ok(TruncationStrategy::LongestFirst),
            "only_first" => Ok(TruncationStrategy::OnlyFirst),
            "only_second" => Ok(TruncationStrategy::OnlySecond),
            "do_not_truncate" => Ok(TruncationStrategy::DoNotTruncate),
            _ => Err("Invalid truncation strategy provided. Must be one of `longest_first`, `only_first`, `only_second` or `do_not_truncate`")
        };
        match truncation_strategy {
            Ok(truncation_strategy) => Ok(self.tokenizer.encode_pair_list(text_list, max_len, &truncation_strategy, stride)),
            Err(e) => Err(exceptions::ValueError::py_err(e))
        }
    }
}


#[pymodule]
fn rust_transformers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBertTokenizer>()?;

    Ok(())
}