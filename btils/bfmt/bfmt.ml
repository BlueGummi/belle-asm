open Sys
open Printf
open Filename

let format_line line =
  if String.contains line ';' then ""
  else if String.ends_with ~suffix:":" line || String.starts_with ~prefix:"." line then line
  else "    " ^ line

let read_file file_path =
  let ic = open_in file_path in
  let rec read_lines acc =
    try
      let line = input_line ic in
      read_lines (line :: acc)
    with End_of_file ->
      close_in ic;
      List.rev acc
  in
  String.concat "\n" (read_lines [])

let read_file_safe file_path =
  try
    Some (read_file file_path)
  with
  | Sys_error _ -> None

let try_rename temp_file_path original_file_path =
  try
    rename temp_file_path original_file_path;
    Some ()
  with
  | Sys_error _ -> None

let remove_file_safe file_path =
  try
    remove file_path
  with
  | Sys_error _ -> ()

let process_input_file input_file_path =
  if not (file_exists input_file_path) then ()
  else
    match read_file_safe input_file_path with
    | None -> ()
    | Some file_contents ->
        let formatted_lines =
          List.map format_line (String.split_on_char '\n' file_contents) in
        let temp_file_path =
          (dirname input_file_path) ^ "/" ^ (basename input_file_path) ^ ".tmp" in
        let oc = open_out temp_file_path in
        List.iter (fprintf oc "%s\n") formatted_lines;
        close_out oc;
        match try_rename temp_file_path input_file_path with
        | None -> remove_file_safe temp_file_path
        | Some () -> ()

let () =
  let command_line_args = Array.to_list Sys.argv in
  match command_line_args with
  | [_] ->
      printf "Usage: %s <input>\n" (basename Sys.argv.(0))
  | _ -> List.iter process_input_file (List.tl command_line_args)
