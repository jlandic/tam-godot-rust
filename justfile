set positional-arguments

base_rust_folder := "rust"
components_folder := base_rust_folder + "/src/ecs/components"
systems_folder := base_rust_folder + "/src/ecs/systems"
sync_folder := base_rust_folder + "/src/ecs/sync"

rust_indent_size := "4"

default:
    just --list

build:
    cargo build

run:
    make run

clippy:
    cargo clippy -- -D warnings

fmt:
    cargo fmt --all -- --check

fmt-fix:
    cargo fmt --all

lint: fmt clippy

generate-component component:
    #!/usr/bin/env ruby
    struct_name = '{{component}}'.split('_').reduce('') { |acc, elt| acc + elt.capitalize }
    File.open('{{components_folder}}/{{component}}.rs', File::EXCL) do |f|
        f.puts 'use bevy_ecs::prelude::*;'
        f.puts ''
        f.puts '#[derive(Component)]'
        f.puts "pub struct #{struct_name} {"
        f.puts '}'
    end

    File.open('{{components_folder}}/mod.rs', 'a') do |f|
        f.puts 'mod {{component}};'
        f.puts "pub use {{component}}::#{struct_name};"
    end

generate-system system:
    #!/usr/bin/env ruby
    def indent(n = 1)
        ' ' * {{rust_indent_size}} * n
    end

    File.open('{{systems_folder}}/{{system}}.rs', File::EXCL) do |f|
        f.puts 'use bevy_ecs::prelude::*;'
        f.puts ''
        f.puts 'use crate::ecs::components::*;'
        f.puts ''
        f.puts 'pub fn {{system}}('
        f.puts "#{indent}query: Query<>,"
        f.puts ') {'
        f.puts indent
        f.puts '}'
    end

    File.open('{{systems_folder}}/mod.rs', 'a') do |f|
        f.puts 'mod {{system}};'
        f.puts 'pub use {{system}}::{{system}};'
    end

generate-sync system:
    #!/usr/bin/env ruby
    def indent(n = 1)
        ' ' * {{rust_indent_size}} * n
    end

    File.open('{{sync_folder}}/{{system}}.rs', File::EXCL) do |f|
        f.puts 'use bevy_ecs::prelude::*;'
        f.puts ''
        f.puts 'use crate::ecs::components::*;'
        f.puts 'use crate::engine::Owner;'
        f.puts ''
        f.puts 'pub fn {{system}}('
        f.puts "#{indent}owner: Res<Owner>,"
        f.puts "#{indent}query: Query<>,"
        f.puts ') {'
        f.puts indent
        f.puts '}'
    end

    File.open('{{sync_folder}}/mod.rs', 'a') do |f|
        f.puts 'mod {{system}};'
        f.puts 'pub use {{system}}::{{system}};'
    end

alias r := run
alias l := lint
alias b := build
alias gc := generate-component
alias gs := generate-system
alias gsc := generate-sync
