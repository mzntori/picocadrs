//! Houses the struct representing a model which is equivalent to all the information a
//! picoCAD file holds.
//!
//! A picoCAD file consists of 3 main parts.
//! - _[`header`](crate::assets::header):_ Contains general settings of the project,
//! like background color or name.
//! Each component is seperated by `;`.
//! Its end is indicated by a newline (`\n`) character, meaning this is always the first line of the
//! file.
//! - _[`meshes`](crate::assets::mesh):_ This is a [`lua table`](https://www.lua.org/pil/2.5.html)
//! holding a list of meshes.
//! The order these are in does not matter.
//! Each mesh itself is also represented as a [`lua table`](https://www.lua.org/pil/2.5.html).
//! Aside from the lua table's closing bracket the end of this section is indicated by a `%`
//! - _[`footer`](crate::assets::footer):_ Holds the texture used for uv mapping.

use crate::{
    assets::{Footer, Header, Mesh},
    error::PicoError,
    paths::projects_path,
};
use rlua::{Lua, Table};
use std::ffi::OsString;
use std::{
    fmt::{Display, Formatter},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

/// A picoCAD model.
///
/// This contains the same information a picoCAD project file does.
/// It is split into three parts.
///
/// - The [`Header`] contains general settings of the project, like background color or name.
/// - After the header there is a list of [`meshes`](Mesh) that combined define the 3-dimensional structure of
/// the model.
/// This part also takes care of uv-mapping.
/// - At the end is the [`Footer`] which holds the texture used for uv-mapping.
///
/// <br/>
///
/// It is important that there is a newline character after the header as well as a '%' before the
/// footer to assure the file can be parsed properly.
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    /// Header of the file.
    pub header: Header,
    /// Meshes, this model consists of.
    pub meshes: Vec<Mesh>,
    /// Footer, holding the texture for uv mapping.
    pub footer: Footer,
}

impl Model {
    /// Loads a model from an absolute path.
    ///
    /// It's recommended to use [`load`](Model::load).
    pub fn load_from_path(path: OsString) -> Result<Model, PicoError> {
        let file_string = std::fs::read_to_string(path)?;

        file_string.parse::<Model>()
    }

    /// Loads a model from a given file-name.
    ///
    /// Returns an error if the users home directory can't be found ([`PicoError::NoHomeDirectory`])
    /// or if file doesn't exist [`PicoError::IO`].
    ///
    /// # Example
    ///
    /// ```
    /// use std::ffi::OsString;
    /// use picocadrs::assets::Model;
    ///
    /// // Requires a valid picoCAD project file in projects folder called "test.txt"
    /// let model = Model::load(OsString::from("test.txt")).unwrap();
    ///
    /// assert_eq!(model.header.name, "test");
    /// ```
    pub fn load(file_name: OsString) -> Result<Model, PicoError> {
        if let Some(mut projects_path) = projects_path() {
            projects_path.push(file_name);
            projects_path.push(".txt");
            Model::load_from_path(projects_path)
        } else {
            Err(PicoError::NoHomeDirectory)
        }
    }

    /// Writes the model to the project file named after the value in [`self.header.name`](Header).
    ///
    /// This means if that field contains the string `my_model` this will be written to
    /// `{result from` [`projects_path`]`}/my_model.txt`.
    ///
    /// Returns errors if files can't be written to.
    ///
    /// Contents of the file will be overwritten.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::Model;
    /// use std::ffi::OsString;
    ///
    /// let mut model = Model::default();
    /// model.header.name = "model_write_example".to_string();
    /// model.write().unwrap();
    ///
    /// let read_model = Model::load(OsString::from("model_write_example")).unwrap();
    ///
    /// assert_eq!(model, read_model);
    /// ```
    pub fn write(&self) -> Result<(), PicoError> {
        let mut path = PathBuf::from(projects_path().unwrap());
        path.push(self.header.name.clone());
        path.set_extension("txt");

        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_string().as_bytes())?;

        Ok(())
    }
}

impl Default for Model {
    /// Creates a new Model with a default header and footer and no meshes.
    ///
    /// # Example
    ///
    /// ```
    /// use picocadrs::assets::{Model, Footer, Header};
    ///
    /// let model = Model::default();
    ///
    /// assert_eq!(model.header, Header::default());
    /// assert_eq!(model.footer, Footer::default());
    /// assert!(model.meshes.is_empty());
    /// ```
    fn default() -> Self {
        Model {
            header: Header::default(),
            meshes: vec![],
            footer: Footer::default(),
        }
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut meshes = String::new();

        for mesh in self.meshes.iter() {
            meshes.push_str(format!("{},", mesh).as_str());
        }
        meshes = meshes.trim_end_matches(',').to_string();

        write!(
            f,
            "{}\n{{\n{}\n}}%\n{}",
            self.header,
            meshes.trim_end_matches(','),
            self.footer
        )
    }
}

impl FromStr for Model {
    type Err = PicoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header_str, meshes_str, footer_str) = seperate_model(s)?;

        let header: Header = header_str.parse()?;
        let footer: Footer = footer_str.parse()?;

        let mut meshes: Vec<Mesh> = vec![];
        let mut lua_result: Result<(), PicoError> = Ok(());

        // We would be fucked without '?' LUL
        let lua = Lua::new();
        lua.context(|ctx| match ctx.load(meshes_str).eval::<Table>() {
            Ok(meshes_table) => {
                for mesh_table_result in meshes_table.sequence_values::<Table>() {
                    match mesh_table_result {
                        Ok(mesh_table) => {
                            let mesh_result = Mesh::try_from(mesh_table);

                            match mesh_result {
                                Ok(mesh) => meshes.push(mesh),
                                Err(parse_error) => {
                                    lua_result = Err(parse_error);
                                    return;
                                }
                            }
                        }
                        Err(lua_err) => {
                            lua_result = Err(PicoError::from(lua_err));
                            return;
                        }
                    }
                }
            }
            Err(lua_err) => {
                lua_result = Err(PicoError::from(lua_err));
            }
        });

        lua_result?;

        Ok(Model {
            header,
            meshes,
            footer,
        })
    }
}

/// Returns header, meshes and footer as their literal strings.
/// If seperators do not exist this will fail.
fn seperate_model(model: &str) -> Result<(&str, &str, &str), PicoError> {
    let (header, rest) = if let Some(split) = model.split_once('\n') {
        split
    } else {
        return Err(PicoError::Split(
            r#"seperate header from meshes with '\n'"#.to_string(),
        ));
    };

    let (meshes, footer) = if let Some(split) = rest.rsplit_once('%') {
        split
    } else {
        return Err(PicoError::Split(
            r#"seperate meshes from footer with '%'"#.to_string(),
        ));
    };

    Ok((header, meshes, footer))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::paths::projects_path;

    #[test]
    fn test_model_parse() {
        dbg!(TEST_FILE.parse::<Model>().unwrap());
    }

    #[test]
    fn test_model_display() {
        assert_eq!(TEST_FILE, TEST_FILE.parse::<Model>().unwrap().to_string())
    }

    #[test]
    fn test_model_default() {
        let model = Model::default();

        assert_eq!(model.header, Header::default());
        assert_eq!(model.footer, Footer::default());
        assert!(model.meshes.is_empty());
    }

    /// Requires a file called `test3.txt` with the contents of [`TEST_FILE`]
    #[test]
    fn test_model_load() {
        let mut path: OsString = projects_path().unwrap();
        path.push("test3.txt");

        assert_eq!(TEST_FILE, Model::load_from_path(path).unwrap().to_string());

        assert_eq!(
            TEST_FILE,
            Model::load(OsString::from("test3")).unwrap().to_string()
        );
    }

    #[test]
    fn test_model_write() {
        let mut model = TEST_FILE.parse::<Model>().unwrap();
        model.header.name = "test_model_write".to_string();
        model.write().unwrap();

        let read_model = Model::load(OsString::from("test_model_write")).unwrap();

        assert_eq!(model, read_model);
    }

    const TEST_FILE: &str = r#"picocad;test3;16;1;0
{
{
 name='plane', pos={0,0,1}, rot={0,0,0},
 v={
  {-1,0,-1},
  {1,0,-1},
  {1,0,1},
  {-1,0,1}
 },
 f={
  {4,3,2,1, c=10, dbl=1, noshade=1, notex=1, prio=1, uv={16.25,0,1.25,0,15.5,2,-0.75,2} }
 }
},{
 name='cube', pos={0,0,0}, rot={0,-0.5,0},
 v={
  {-0.5,-0.5,-0.5},
  {0.5,-0.5,-0.5},
  {0.5,0.5,-0.5},
  {-0.5,0.5,-0.5},
  {-0.5,-0.5,0.5},
  {0.5,-0.5,0.5},
  {0.5,0.5,0.5},
  {-0.5,0.5,0.5}
 },
 f={
  {1,2,3,4, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {6,5,8,7, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {5,6,2,1, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {5,1,4,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {2,6,7,3, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} },
  {4,3,7,8, c=11, uv={5.5,0.5,6.5,0.5,6.5,1.5,5.5,1.5} }
 }
}
}%
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333ccccddddccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbbddddccccddddcccceeeeffffeeeeffff6666777766667777666655556666555500000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
00000000eeee8888eeee8888aaaa9999aaaa9999bbbb3333bbbb3333cccc4444ccccddddffffeeeeffffeeee7777666677776666555566665555666600000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
000000008888eeee8888eeee9999aaaa9999aaaa3333bbbb3333bbbb2222cccc1111cccceeeeffffeeeeffff6666777766667777666655556666555500000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
"#;
}
