# Stained Glass Pattern With Templates

## Tool and Serving

This is a self-hosted stained or leaded glass design tool that provides basic shape and free-drawing. It provides several paper sizes, common to home use, and saves the designs as JSON formatted .gls files (fun play on glass). 

I am running this on a Pi SBC server for my own use using nginx. The index.html included here references the /templates directory to provide template and sample designs. Files can be saved and imported locally, printed, and exported to .pdf for saving, printing and sharing more easily.

This tool is usable for hobbyists, and is being further developed for my own purposes, but also to share as a usable way to design simple stained glass. Please share your own forks if you do something more with it. 

Because this is intended for self-hosting, I have made no effort at this time to secure anything. Please be careful if you plan to share access to anything hosting this tool, and ensure that you add controls to ensure that users cannot save to the /templates directory. In the future I plan to tackle that part, but I wanted a tool for myself right now.

##/etc/nginx/sites-available/glass.local

Included the configuration you need for nginx with the /templates calls.

##Templates

The /templates directory contains pre-formatted `.gls` template files designed for loading directly into the tool.

The `.gls` file format represents vector-based stained glass patterns. Each file contains canvas metadata, page orientation settings, and line segment definitions formatted for exact pixel-to-inch rendering on standard paper sizes.

---

## File Schema Reference

Patterns use JSON object structures with absolute pixel coordinate endpoints calculated at **96 DPI** (defaulting to 8.5" × 11" Letter format).

### Core Structure

```json
{
  "projectName": "Pattern Name",
  "constructionType": "foil", // Options: "foil" or "came"
  "unit": "in",
  "paper": "letter",
  "orientation": "portrait",
  "elements": [
    {
      "type": "line",
      "p1": { "x": 100, "y": 100 },
      "p2": { "x": 300, "y": 100 }
    }
  ]
}
