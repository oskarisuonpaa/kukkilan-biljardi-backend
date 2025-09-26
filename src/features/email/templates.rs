use chrono::{DateTime, Utc};

pub struct BookingConfirmationData {
    pub customer_name: String,
    pub calendar_name: String,
    pub table_type: String,
    pub booking_date: String,
    pub start_time: String,
    pub end_time: String,
    pub duration_hours: i64,
    pub total_price: i32, // in cents
    pub booking_id: u32,
}

pub fn generate_booking_confirmation_html(data: &BookingConfirmationData) -> String {
    let price_euros = data.total_price as f64 / 100.0;
    
    format!(
        r#"<!DOCTYPE html>
<html lang="fi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Varausvahvistus - Kukkilan Biljardi</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f9f9f9;
        }}
        .container {{
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }}
        .header {{
            text-align: center;
            border-bottom: 3px solid #2c5530;
            padding-bottom: 20px;
            margin-bottom: 30px;
        }}
        .header h1 {{
            color: #2c5530;
            margin: 0;
            font-size: 28px;
        }}
        .booking-details {{
            background: #f8f9fa;
            padding: 20px;
            border-radius: 8px;
            margin: 20px 0;
        }}
        .detail-row {{
            display: flex;
            justify-content: space-between;
            margin: 10px 0;
            padding: 5px 0;
            border-bottom: 1px solid #e9ecef;
        }}
        .detail-row:last-child {{
            border-bottom: none;
        }}
        .label {{
            font-weight: bold;
            color: #495057;
        }}
        .value {{
            color: #212529;
        }}
        .highlight {{
            background: #d4edda;
            padding: 15px;
            border-radius: 5px;
            border-left: 4px solid #28a745;
            margin: 20px 0;
        }}
        .footer {{
            margin-top: 30px;
            padding-top: 20px;
            border-top: 1px solid #dee2e6;
            text-align: center;
            color: #6c757d;
            font-size: 14px;
        }}
        .contact-info {{
            margin: 20px 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎱 Kukkilan Biljardi</h1>
            <p>Varausvahvistus</p>
        </div>
        
        <p>Hei <strong>{}</strong>!</p>
        
        <p>Kiitos varauksestasi! Varauksen tiedot on vahvistettu ja odotamme sinua Kukkilan Biljardiin.</p>
        
        <div class="highlight">
            <h3>📅 Varausnumero: #{}</h3>
        </div>
        
        <div class="booking-details">
            <h3>Varauksen tiedot:</h3>
            
            <div class="detail-row">
                <span class="label">Pöytä:</span>
                <span class="value">{} ({})</span>
            </div>
            
            <div class="detail-row">
                <span class="label">Päivä:</span>
                <span class="value">{}</span>
            </div>
            
            <div class="detail-row">
                <span class="label">Aika:</span>
                <span class="value">{} - {}</span>
            </div>
            
            <div class="detail-row">
                <span class="label">Kesto:</span>
                <span class="value">{} tuntia</span>
            </div>
            
            <div class="detail-row">
                <span class="label">Kokonaishinta:</span>
                <span class="value"><strong>{:.2} €</strong></span>
            </div>
        </div>
        
        <div class="contact-info">
            <h3>📍 Yhteystiedot</h3>
            <p><strong>Kukkilan Biljardi</strong><br>
            Osoite: [Osoite täydennetään]<br>
            Puhelin: [Puhelinnumero täydennetään]<br>
            Sähköposti: [Sähköposti täydennetään]</p>
        </div>
        
        <div class="highlight">
            <h4>⚠️ Tärkeää:</h4>
            <ul>
                <li>Saavu paikalle ajoissa - myöhästymisestä voidaan veloittaa</li>
                <li>Peruutukset tulee tehdä vähintään 2 tuntia ennen varattua aikaa</li>
                <li>Tuo mukanasi henkilöllisyystodistus</li>
                <li>Maksu suoritetaan paikan päällä</li>
            </ul>
        </div>
        
        <div class="footer">
            <p>Tämä on automaattinen viesti. Älä vastaa tähän sähköpostiin.</p>
            <p>Jos tarvitset apua, ota yhteyttä puhelimitse tai käy paikan päällä.</p>
        </div>
    </div>
</body>
</html>"#,
        data.customer_name, // name in greeting
        data.booking_id,    // booking number
        data.calendar_name, // table name
        data.table_type,    // table type
        data.booking_date,  // date
        data.start_time,    // start time
        data.end_time,      // end time
        data.duration_hours, // duration
        price_euros         // price
    )
}

pub fn generate_booking_confirmation_text(data: &BookingConfirmationData) -> String {
    let price_euros = data.total_price as f64 / 100.0;
    
    format!(
        r#"KUKKILAN BILJARDI - VARAUSVAHVISTUS

Hei {}!

Kiitos varauksestasi! Varauksen tiedot on vahvistettu ja odotamme sinua Kukkilan Biljardiin.

VARAUSNUMERO: #{}

VARAUKSEN TIEDOT:
- Pöytä: {} ({})
- Päivä: {}
- Aika: {} - {}
- Kesto: {} tuntia
- Kokonaishinta: {:.2} €

YHTEYSTIEDOT:
Kukkilan Biljardi
Osoite: [Osoite täydennetään]
Puhelin: [Puhelinnumero täydennetään]
Sähköposti: [Sähköposti täydennetään]

TÄRKEÄÄ:
- Saavu paikalle ajoissa - myöhästymisestä voidaan veloittaa
- Peruutukset tulee tehdä vähintään 2 tuntia ennen varattua aikaa
- Tuo mukanasi henkilöllisyystodistus
- Maksu suoritetaan paikan päällä

Tämä on automaattinen viesti. Älä vastaa tähän sähköpostiin.
Jos tarvitset apua, ota yhteyttä puhelimitse tai käy paikan päällä.

Kukkilan Biljardi"#,
        data.customer_name, // name in greeting
        data.booking_id,    // booking number
        data.calendar_name, // table name
        data.table_type,    // table type
        data.booking_date,  // date
        data.start_time,    // start time
        data.end_time,      // end time
        data.duration_hours, // duration
        price_euros         // price
    )
}