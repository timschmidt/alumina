use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Database {
    points_to_plot: Vec<[f64; 2]>
}

impl super::Demo for Database {
    fn name(&self) -> &'static str {
        "🗠 Database"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(1200.0, 800.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Database {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {

        let database_definition = array(
            "requests" => array(
                "id INTEGER PRIMARY KEY" => "Request ID (Primary Key)",
                "type TEXT" => "Request Type",
                "tool TEXT" => "Tool ID",
                "workcenter TEXT" => "Workcenter Number",
                "originator TEXT" => "Person originating maintenance request",
                "cause TEXT" => "Cause of the problem",
                "date TEXT" => "Date entered",
                "due_date TEXT" => "Tool needed by this date",
                "comment TEXT" => "Comment",
                "active TEXT" => "Timestamp added when Authorized"
            ),
            "services" => array(
                "id INTEGER PRIMARY KEY" => "Service ID (Primary Key)",
                "request TEXT" => "Request which initiated service",
                "serviced_by TEXT" => "Serviced by",
                "date TEXT" => "Date Serviced",
                "comment TEXT" => "Comment",
                "pick_up TEXT" => "Pick-up Date",
                "delivered TEXT" => "Delivered Date",
                "components TEXT" => "Components used in service",
                "check_list TEXT" => "Check List",
                "reference TEXT" => "Generic Reference",
                "completed" => "Timestamp added when Service completed"
            ),
            "users" => array(
                "id INTEGER PRIMARY KEY" => "User ID (Primary Key)",
                "username TEXT" => "User Name",
                "password TEXT" => "Password",
                "comment TEXT" => "Comment",
                "role TEXT" => "Security clearance",
                "url_id TEXT" => "User-visible session ID",
                "session_date TEXT" => "Date url_id was assigned",
                "page TEXT" => "Displayed page",
                "page_data TEXT" => "Extra data about page"
            ),
            "tools" => array(
                "id INTEGER PRIMARY KEY" => "Tool ID (Primary Key)",
                "check_list TEXT" => "Service Check List",
                "tool TEXT" => "Tool tracking number",
                "hits_pm TEXT" => "Hits before scheduled PM",
                "hits_since_pm TEXT" => "Hits since last PM service",
                "description TEXT" => "Comment",
                "verified TEXT" => "Verified",
                "location_run TEXT" => "Run Location",
                "location_storage TEXT" => "Storage Location",
                "size_left2right TEXT" => "Left to right size",
                "size_front2back TEXT" => "Front to back size",
                "size_shut_height TEXT" => "Shut Height size",
                "workcenter_primary TEXT" => "Primary Workcenter",
                "workcenter_secondary TEXT" => "Secondary Workcenter",
                "weight_top TEXT" => "Top Weight",
                "weight_total TEXT" => "Total Weight",
                "status TEXT" => "Status",
                "time_setup TEXT" => "Setup time",
                "manufacturer TEXT" => "Manufacturer",
                "operation_number TEXT" => "Operation Number",
                "asset_number TEXT" => "Asset number",
                "job_number TEXT" => "Job Number",
                "owner TEXT" => "Owner",
                "tonnage TEXT" => "Press tonnage required",
                "spm TEXT" => "Strokes Per Minute",
                "material_width TEXT" => "Material Width",
                "feed_pitch TEXT" => "Feed Pitch",
                "tool_customer" => "Customers tool number",
                "notes" => "General Notes"
            ),
            "part_numbers" => array(
                "id INTEGER PRIMARY KEY" => "Part Num ID (Primary Key)",
                "part_number TEXT" => "Part Number",
                "tool TEXT" => "Associated Tool"
            ),
            "causes" => array(
                "id INTEGER PRIMARY KEY" => "Cause ID (Primary Key)",
                "cause TEXT" => "Cause of service request"
            ),
            "types" => array(
                "id INTEGER PRIMARY KEY" => "Type ID (Primary Key)",
                "type TEXT" => "Type of service request"
            ),
            "workcenters" => array(
                "id INTEGER PRIMARY KEY" => "Workcenter ID (Primary Key)",
                "workcenter TEXT" => "Workcenter number / name",
                "comment TEXT" => "Comment",
                "make TEXT" => "Make",
                "type TEXT" => "Type",
                "stroke TEXT" => "Stroke",
                "stock_thickness TEXT" => "(x3)Stock Thickness",
                "stock_width TEXT" => "(x3)Stock width",
                "spm TEXT" => "Strokes Per Minute",
                "shut_height_min TEXT" => "Shut Height Min",
                "shut_height_max TEXT" => "Shut Height Max",
                "shut_height_preferred TEXT" => "Shut Height Preferred",
                "size_bed_front2back TEXT" => "Bed Size Front to Back",
                "size_bed_left2right TEXT" => "Bed Size Left to Right",
                "size_ram_front2back TEXT" => "RAM Size Front to Back",
                "size_ram_left2right TEXT" => "RAM Size Left to Right",
                "feeder_stroke TEXT" => "Feeder Stroke",
                "feed_lvl_preferred TEXT" => "Preferred Feed Lvl",
                "od_min TEXT" => "OD Min",
                "od_max TEXT" => "OD Max",
                "id_min TEXT" => "ID Min",
                "id_max TEXT" => "ID Max",
                "tonnage_max TEXT" => "Max tonnage",
                "exceptions TEXT" => "Exceptions",
                "location" => "Location"
            ),
            "photos" => array(
                "id INTEGER PRIMARY KEY" => "Photo ID (Primary Key)",
                "filename TEXT" => "Photo filename",
                "comment TEXT" => "Comment",
                "tool TEXT" => "Tool IDs",
                "services TEXT" => "Service IDs",
                "requests TEXT" => "Request IDs",
                "workcenters TEXT" => "Workcenter IDs"
            ),
        );

    }
}