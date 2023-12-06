use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AboutBullseye)]
pub fn about_bullseye() -> Html {
    html!(
        <Bullseye>
            <AboutModal
                background_image_src="/public/images/background.png"
                brand_image_src="/public/images/logo.png"
                brand_image_alt="Akiw logo"
                product_name="Akiw"
                trademark="Copyright © 2023 Akiw"
            >

            <p>{ env!("CARGO_PKG_DESCRIPTION") }</p>
            <br />

            <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                <DescriptionGroup term="Name">{env!("CARGO_PKG_NAME")}</DescriptionGroup>
                <DescriptionGroup term="Version">{env!("CARGO_PKG_VERSION")}</DescriptionGroup>
                <DescriptionGroup term="License">{env!("CARGO_PKG_LICENSE")}</DescriptionGroup>
            </DescriptionList>

            </AboutModal>
        </Bullseye>
    )
}
