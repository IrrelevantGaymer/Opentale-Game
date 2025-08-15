/// We generate a list of block_builders
#[macro_export]
macro_rules! table {
    (
        $slice:path, $build:path,
        enum $enum:ident,
        static $table:ident = {
            $(let $block_name:ident : $block_type:ty = $block_expr:expr ;)*
        }
    ) => {
        $crate::assert_items_define!(
            $slice, 
            $($block_name : $block_type),*
        );
        $crate::items_define!(
            1, 1,
            $($block_name : $block_type = $block_expr),*
        );

        $crate::table_define!(
            $table, $slice, 
            $($block_name),*
        );

        $crate::enum_define!(
            $enum, $slice, $build,
            $($block_name : $block_type),*
        );
    };
}

#[macro_export]
macro_rules! assert_items_define {
    (
        $slice:path, 
        $first_name:ident : $first_type:ty
        $(, $rest_name:ident : $rest_type:ty)*
    ) => {
        static_assertions::assert_impl_all!(
            $first_type: $crate::world_generation
                ::blocks
                ::buildable
                ::Buildable, 
            $slice
        );
        $crate::assert_items_define!(
            $slice, 
            $($rest_name : $rest_type),*
        );
    };
    ($slice:path, ) => {

    }
}

#[macro_export]
macro_rules! items_define {
    (
        $id:expr, $idx:expr,
        $first_name:ident : $first_type:ty = $first_expr:expr 
        $(, $rest_name:ident : $rest_type:ty = $rest_expr:expr)*
    ) => {
        #[allow(non_upper_case_globals)]
        static $first_name: $first_type = 
            <
                $first_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::Buildable
            >::with_id(
                $first_expr,
                $id
            );
        $crate::items_define!(
            $id + <
                $first_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::Buildable
            >::get_id_span(),
            $idx + <
                $first_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::Buildable
            >::get_texture_size(),
            $($rest_name : $rest_type = $rest_expr),*
        );
    };
    ($id:expr, $idx:expr,) => {
        
    };
}

#[macro_export]
macro_rules! table_define {
    (
        $table_name:ident, $slice:path, 
        $($block_name:ident),*
    ) => {
        pub static $table_name : $crate::world_generation
            ::blocks
            ::table
            ::BuilderTable<dyn $slice> 
        = $crate::world_generation::blocks::table::BuilderTable(&[
            $(& $block_name),*
        ]);
    }
}

#[macro_export]
macro_rules! enum_define {
    (
        $enum_name:ident, 
        $slice:path, $build:path, 
        $($block_name:ident : $block_type:ty),*
    ) => {
        pub enum $enum_name {
            Air(()),
            $($block_name(<
                $block_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::HasBuildVariants
            >::Variants)),*
        }

        impl $enum_name {
            pub fn inner_id(&self) -> usize {
                match self {
                    $enum_name::Air(()) => 0,
                    $(
                        $enum_name::$block_name(inner) => <
                            <
                                $block_type as 
                                $crate::world_generation
                                    ::blocks
                                    ::buildable
                                    ::HasBuildVariants
                            >::Variants as 
                            $crate::world_generation
                                ::blocks
                                ::as_id
                                ::AsId
                        >::to_id(inner)
                    ),*
                }
            }

            #[allow(unused_assignments)]
            pub fn index(&self) -> usize {
                if let $enum_name::Air(_) = self {
                    return 0;
                }
                let mut index = 1;
                $(
                    if let $enum_name::$block_name(_) = self {
                        return index;
                    }
                    index += 1;
                )*
                unreachable!();
            }
        }

        impl $crate::world_generation::blocks::as_id::AsId for $enum_name {
            type Name = &'static str;
            const NAME: Self::Name = stringify!($enum_name);

            #[allow(unused_assignments)]
            fn from_id(id: usize) -> Self {
                $crate::from_id_inner!(
                    $enum_name, 
                    id, 
                    $($block_name : $block_type),*
                );
            }

            #[allow(unused_assignments)]
            fn to_id(&self) -> usize {
                $crate::to_id_inner!(
                    $enum_name, 
                    self, 
                    $($block_name : $block_type),*
                );
            }

            fn get_id_span() -> usize {
                1 + $crate::get_id_span_inner!($($block_type),*)
            }

            fn to_string(&self) -> String {
                match self {
                    $enum_name::Air(()) => "Air".to_string(),
                    $(
                        $enum_name::$block_name(inner) => if <
                            <
                                $block_type as 
                                $crate::world_generation
                                    ::blocks
                                    ::buildable
                                    ::HasBuildVariants
                            >::Variants as 
                            $crate::world_generation
                                ::blocks
                                ::as_id
                                ::AsId
                        >::get_id_span() == 1 {
                            stringify!($block_name).to_string()
                        } else {
                            stringify!($block_name).to_string() 
                                + "[" 
                                + inner.to_string().as_str() 
                                + "]"
                        },
                    )*
                }
            }
        }

        impl std::ops::Index<$enum_name> for $crate::world_generation::blocks::table::Table<$build> {
            type Output = $build;

            fn index(&self, idx: $enum_name) -> &Self::Output {
                let index = idx.index();
                unsafe {
                    self.get_unchecked(index)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! from_id_inner {
    ($enum_name:ident, $id:expr, $($block_name:ident : $block_type:ty),*) => {
        if $id == 0 {
            return $enum_name::Air(());
        }
        
        let mut offset = 1;

        $(
            let size = <
                <
                    $block_type as 
                    $crate::world_generation
                        ::blocks
                        ::buildable
                        ::HasBuildVariants
                >::Variants as 
                $crate::world_generation
                    ::blocks
                    ::as_id
                    ::AsId
            >::get_id_span();
            if $id >= offset && $id < offset + size {
                let variant_id = $id - offset;
                return $enum_name::$block_name(
                    <_>::from_id(variant_id)
                );
            }
            offset += size;
        )*

        panic!("{} is an invalid Id for {}", $id, stringify!($enum_name));
    }
}

#[macro_export]
macro_rules! to_id_inner {
    (
        $enum_name:ident, 
        $self:ident, 
        $($block_name:ident : $block_type:ty),*
    ) => {
        let mut offset = 0;
        if let $enum_name::Air(()) = $self {
            return offset;
        }
        offset += 1;
        $(
            if let $enum_name::$block_name(inner) = $self {
                return offset + inner.to_id();
            }
            offset += <
                <
                    $block_type as 
                    $crate::world_generation
                        ::blocks
                        ::buildable
                        ::HasBuildVariants
                >::Variants as 
                $crate::world_generation
                    ::blocks
                    ::as_id
                    ::AsId
            >::get_id_span();
        )*
        unreachable!();
    };
}

#[macro_export]
macro_rules! get_id_span_inner {
    ($first_type:ty, $($rest_type:ty),+) => {
        <
            <
                $first_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::HasBuildVariants
            >::Variants as 
        $crate::world_generation
            ::blocks
            ::as_id
            ::AsId
        >::get_id_span() + $crate::get_id_span_inner!($($rest_type),*)
    };
    ($block_type:ty) => {
        <
            <
                $block_type as 
                $crate::world_generation
                    ::blocks
                    ::buildable
                    ::HasBuildVariants
            >::Variants as 
            $crate::world_generation
                ::blocks
                ::as_id
                ::AsId
        >::get_id_span() 
    };
    () => {
        0
    };
}

#[macro_export]
macro_rules! id {
    ($id:path {$($field:ident : $v:expr),*}) => {
        $id ($crate::id_inner!($($v),*))
    };
    ($id:path) => {
        $id(())
    };
}

#[macro_export]
macro_rules! id_inner {
    () => {
        ()
    };
    ($v:expr $(,)?) => {
        ($v, ())
    };
    ($v:expr, $($vs:expr),+) => {
        ($v, $crate::id_inner!($($vs),*))
    };
}