pub mod desktop;
pub mod mobile;

use desktop::generate_desktop_css;
use mobile::generate_mobile_css;

pub fn generate_css(theme: &str) -> String {
    let theme_vars = match theme {
        "purple" => r#"        :root {
            --background: 310 100% 98%;
            --foreground: 270 15% 15%;
            --card: 310 100% 98%;
            --card-foreground: 270 15% 15%;
            --popover: 310 100% 98%;
            --popover-foreground: 270 15% 15%;
            --primary: 270 91% 65%;
            --primary-foreground: 310 100% 98%;
            --secondary: 270 20% 92%;
            --secondary-foreground: 270 15% 15%;
            --muted: 270 20% 92%;
            --muted-foreground: 270 10% 55%;
            --accent: 270 20% 92%;
            --accent-foreground: 270 15% 15%;
            --destructive: 0 84.2% 60.2%;
            --destructive-foreground: 310 100% 98%;
            --border: 270 20% 88%;
            --input: 270 20% 88%;
            --ring: 270 91% 65%;
            --link: 50 100% 40%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 270 20% 12%;
            --foreground: 310 40% 92%;
            --card: 270 20% 12%;
            --card-foreground: 310 40% 92%;
            --popover: 270 20% 12%;
            --popover-foreground: 310 40% 92%;
            --primary: 270 91% 75%;
            --primary-foreground: 270 20% 12%;
            --secondary: 270 15% 20%;
            --secondary-foreground: 310 40% 92%;
            --muted: 270 15% 20%;
            --muted-foreground: 270 10% 65%;
            --accent: 270 15% 20%;
            --accent-foreground: 310 40% 92%;
            --destructive: 0 62.8% 50%;
            --destructive-foreground: 310 40% 92%;
            --border: 270 15% 20%;
            --input: 270 15% 20%;
            --ring: 270 91% 75%;
            --link: 50 100% 60%;
        }"#,
        "vibrant" => r#"        :root {
            --background: 45 100% 92%;
            --foreground: 220 30% 25%;
            --card: 50 80% 88%;
            --card-foreground: 220 30% 25%;
            --popover: 50 80% 88%;
            --popover-foreground: 220 30% 25%;
            --primary: 200 100% 50%;
            --primary-foreground: 0 0% 98%;
            --secondary: 120 40% 75%;
            --secondary-foreground: 220 30% 25%;
            --muted: 60 60% 85%;
            --muted-foreground: 220 20% 40%;
            --accent: 330 80% 70%;
            --accent-foreground: 0 0% 98%;
            --destructive: 10 80% 60%;
            --destructive-foreground: 0 0% 98%;
            --border: 40 50% 80%;
            --input: 40 50% 80%;
            --ring: 200 100% 50%;
            --link: 320 80% 50%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 220 25% 20%;
            --foreground: 45 85% 85%;
            --card: 215 20% 25%;
            --card-foreground: 45 85% 85%;
            --popover: 215 20% 25%;
            --popover-foreground: 45 85% 85%;
            --primary: 200 90% 65%;
            --primary-foreground: 220 25% 20%;
            --secondary: 120 30% 45%;
            --secondary-foreground: 45 85% 85%;
            --muted: 210 15% 30%;
            --muted-foreground: 45 40% 70%;
            --accent: 330 70% 60%;
            --accent-foreground: 220 25% 20%;
            --destructive: 10 70% 55%;
            --destructive-foreground: 45 85% 85%;
            --border: 210 20% 35%;
            --input: 210 20% 35%;
            --ring: 200 90% 65%;
            --link: 320 80% 60%;
        }"#,
        _ => r#"        :root {
            --background: 0 0% 100%;
            --foreground: 222.2 84% 4.9%;
            --card: 0 0% 100%;
            --card-foreground: 222.2 84% 4.9%;
            --popover: 0 0% 100%;
            --popover-foreground: 222.2 84% 4.9%;
            --primary: 222.2 47.4% 11.2%;
            --primary-foreground: 210 40% 98%;
            --secondary: 210 40% 96%;
            --secondary-foreground: 222.2 47.4% 11.2%;
            --muted: 210 40% 96%;
            --muted-foreground: 215.4 16.3% 46.9%;
            --accent: 210 40% 96%;
            --accent-foreground: 222.2 47.4% 11.2%;
            --destructive: 0 84.2% 60.2%;
            --destructive-foreground: 210 40% 98%;
            --border: 214.3 31.8% 91.4%;
            --input: 214.3 31.8% 91.4%;
            --ring: 222.2 84% 4.9%;
            --link: 120 100% 35%;
            --radius: 0.5rem;
        }

        [data-theme="dark"] {
            --background: 222.2 84% 4.9%;
            --foreground: 210 40% 98%;
            --card: 222.2 84% 4.9%;
            --card-foreground: 210 40% 98%;
            --popover: 222.2 84% 4.9%;
            --popover-foreground: 210 40% 98%;
            --primary: 210 40% 98%;
            --primary-foreground: 222.2 47.4% 11.2%;
            --secondary: 217.2 32.6% 17.5%;
            --secondary-foreground: 210 40% 98%;
            --muted: 217.2 32.6% 17.5%;
            --muted-foreground: 215 20.2% 65.1%;
            --accent: 217.2 32.6% 17.5%;
            --accent-foreground: 210 40% 98%;
            --destructive: 0 62.8% 30.6%;
            --destructive-foreground: 210 40% 98%;
            --border: 217.2 32.6% 17.5%;
            --input: 217.2 32.6% 17.5%;
            --ring: 212.7 26.8% 83.9%;
            --link: 120 100% 50%;
        }"#,
    };

    format!("{}
{}
{}", theme_vars, generate_desktop_css(), generate_mobile_css())
}