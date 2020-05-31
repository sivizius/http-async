/// Use but ignore something.
macro_rules! instead
{
  (
    $_t:tt
    $This:expr
  )
  =>  { $This };
}

/// Beautiful Discrete State Machine
macro_rules! bdsm
{
  {
    $StateType:ident => $Transition:ident ( $(  $InputType:tt )*  ) -> $ResultType:ty,
    $(
      $State:ident => $Function:expr,
    )*
  }
  =>  {
        //pub ( super )
        #[derive(Clone,Copy,Debug)]
        pub ( super )
        enum          $StateType
        {
          $( $State, )*
        }

        impl          $StateType
        {
          pub ( super )
          const $Transition:
          [
            &'static dyn Fn ( $(  $InputType  )*  ) -> $ResultType;
            0 $( + instead!  ( $State  1 ) )*
          ]
          =   [
                $( $Function, )*
              ];
        }
      };
}
