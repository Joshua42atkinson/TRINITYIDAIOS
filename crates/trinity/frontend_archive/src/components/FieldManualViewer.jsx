import React, { useState, useRef, useEffect, useMemo } from 'react';
import { marked } from 'marked';

function splitIntoPages(markdown, maxChars = 1000) {
    if (!markdown || markdown.trim().length === 0) return [''];
    const rawBlocks = markdown.split(/\n\n+/).filter(b => b.trim().length > 0);
    const pages = [];
    let currentBlocks = [];
    let currentLen = 0;

    for (let i = 0; i < rawBlocks.length; i++) {
        const block = rawBlocks[i].trim();
        const isTable = block.includes('|') && block.includes('---');
        const isHeading = block.startsWith('# ');
        const isImage = block.startsWith('![');

        if (isHeading) {
            // Force a page break before top-level headings
            if (currentBlocks.length > 0) {
                pages.push(currentBlocks.join('\n\n'));
                currentBlocks = [];
                currentLen = 0;
            }
        }

        if (isTable || isImage) {
            if (currentBlocks.length > 0) {
                pages.push(currentBlocks.join('\n\n'));
                currentBlocks = [];
                currentLen = 0;
            }
            pages.push(block);
            continue;
        }

        if (currentLen + block.length > maxChars && currentBlocks.length > 0) {
            pages.push(currentBlocks.join('\n\n'));
            currentBlocks = [block];
            currentLen = block.length;
        } else {
            currentBlocks.push(block);
            currentLen += block.length;
        }
    }
    if (currentBlocks.length > 0) pages.push(currentBlocks.join('\n\n'));
    return pages.length > 0 ? pages : [''];
}

export default function FieldManualViewer({ onBack }) {
    const [spreadIndex, setSpreadIndex] = useState(0);
    const [flipAnime, setFlipAnime] = useState('');
    const [pagesContent, setPagesContent] = useState([]);
    const flipTimer = useRef(null);

    useEffect(() => {
        fetch('/docs/ASK_PETE_FIELD_MANUAL.md')
            .then(res => res.text())
            .then(text => {
                const chunks = splitIntoPages(text, 1000);
                
                const pageObjs = chunks.map((chunk, idx) => {
                    const isTitle = chunk.startsWith('# **ASK PETE');
                    return {
                        content: chunk,
                        globalPage: idx + 1,
                        isTitlePage: isTitle,
                    };
                });
                setPagesContent(pageObjs);
            })
            .catch(err => console.error(err));
    }, []);

    const spreads = useMemo(() => {
        const spreadArray = [];
        let i = 0;
        while (i < pagesContent.length) {
            const page = pagesContent[i];
            
            // If it's the title page, we want it on the right to simulate a book cover
            if (page.isTitlePage) {
                spreadArray.push([ null, page ]);
                i += 1;
            } else {
                const leftPage = page;
                const rightPage = i + 1 < pagesContent.length ? pagesContent[i + 1] : null;
                spreadArray.push([ leftPage, rightPage ]);
                i += rightPage ? 2 : 1;
            }
        }
        return spreadArray;
    }, [pagesContent]);

    const handleFlip = (direction, targetIdx) => {
        setFlipAnime(direction);
        if (flipTimer.current) clearTimeout(flipTimer.current);
        flipTimer.current = setTimeout(() => setFlipAnime(''), 600);
        setTimeout(() => setSpreadIndex(targetIdx), 300);
    };

    const nextSlide = () => { if (spreadIndex < spreads.length - 1) handleFlip('flip-forward', spreadIndex + 1); };
    const prevSlide = () => { if (spreadIndex > 0) handleFlip('flip-backward', spreadIndex - 1); };

    const renderPageContent = (page) => {
        if (!page) return null;
        let htmlContent = marked.parse(page.content);
        
        // Auto-fix image paths based on local serving
        htmlContent = htmlContent.replace(/src="\/images\//g, 'src="/docs/images/');

        return (
            <div className={`dnd-markdown ${page.isTitlePage ? 'title-page-markdown' : ''}`}>
                <div dangerouslySetInnerHTML={{ __html: htmlContent }} />
            </div>
        );
    };

    const currentSpread = spreads[spreadIndex] || [null, null];
    const leftPage = currentSpread[0];
    const rightPage = currentSpread[1];

    if (pagesContent.length === 0) {
        return (
            <div className="handbook-container">
                <button onClick={onBack} className="stow-btn">← Stow Manual</button>
                <div style={{ color: '#CFB991', fontFamily: '"Inter", sans-serif' }}>Retrieving Field Manual from servers...</div>
            </div>
        );
    }

    return (
        <div className="handbook-container">
            <button onClick={onBack} className="stow-btn">← Stow Manual</button>
            <div className="page-counter">Spread {spreadIndex + 1} of {spreads.length}</div>

            <div className={`book-spread ${flipAnime}`}>
                <div className="book-spine" />

                {/* Left Page */}
                <div className="book-page left-page content-bg">
                    {renderPageContent(leftPage)}
                    {leftPage && <div className="page-watermark-left">{leftPage.globalPage}</div>}
                </div>

                {/* Right Page */}
                <div className="book-page right-page content-bg">
                    {renderPageContent(rightPage)}
                    {rightPage && <div className="page-watermark-right">{rightPage.globalPage}</div>}
                </div>
            </div>

            <div className="audio-bar" style={{ justifyContent: 'center' }}>
                <div className="nav-group">
                    <button onClick={prevSlide} disabled={spreadIndex === 0}
                        className={`nav-btn ${spreadIndex === 0 ? 'nav-btn--disabled' : ''}`}>
                        ◀ Turn Back
                    </button>
                    <button onClick={nextSlide} disabled={spreadIndex === spreads.length - 1}
                        className={`nav-btn ${spreadIndex === spreads.length - 1 ? 'nav-btn--disabled' : ''}`}>
                        Turn Page ▶
                    </button>
                </div>
            </div>

            <style dangerouslySetInnerHTML={{__html: `
                @import url('https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Inter:wght@400;500;600&display=swap');

                .handbook-container {
                    grid-column: 1 / -1; grid-row: 2;
                    display: flex; flex-direction: column;
                    align-items: center; justify-content: center;
                    background: radial-gradient(circle at center, #1a1510 0%, #050403 100%);
                    height: 100%; width: 100%; overflow: hidden;
                    padding: 16px; position: relative;
                }

                .stow-btn {
                    position: absolute; top: 24px; left: 40px; z-index: 50;
                    padding: 10px 20px; background: rgba(15,12,8,0.95);
                    border: 1px solid #CFB991; color: #CFB991;
                    font-family: 'Cinzel', serif; font-size: 14px; font-weight: 700; cursor: pointer;
                    border-radius: 4px; transition: all 0.3s ease;
                    box-shadow: 0 4px 15px rgba(0,0,0,0.5), inset 0 0 10px rgba(207,185,145,0.1);
                }
                .stow-btn:hover { background: rgba(207,185,145,0.2); box-shadow: 0 4px 20px rgba(207,185,145,0.2), inset 0 0 15px rgba(207,185,145,0.3); }
                .page-counter {
                    position: absolute; top: 24px; right: 40px; z-index: 50;
                    color: #CFB991; font-family: 'Inter', sans-serif; font-size: 14px; font-weight: 600;
                    background: rgba(15,12,8,0.95); padding: 8px 20px;
                    border-radius: 4px; border: 1px solid rgba(207,185,145,0.4);
                    box-shadow: 0 4px 15px rgba(0,0,0,0.5);
                }

                .book-spread {
                    display: flex; width: 100%; max-width: 1500px; height: calc(100% - 90px);
                    background: #2a2015; /* Base for empty pages */
                    box-shadow: 0 40px 80px rgba(0,0,0,0.9), inset 0 0 100px rgba(139,115,85,0.3), 0 0 0 4px #5a4b3c, 0 0 0 6px #111;
                    border-radius: 6px 12px 12px 6px;
                    position: relative; overflow: hidden;
                    transition: opacity 0.7s cubic-bezier(0.4, 0.0, 0.2, 1), transform 0.7s cubic-bezier(0.4, 0.0, 0.2, 1);
                }
                .flip-forward { opacity: 0; transform: perspective(2000px) rotateY(-10deg) translateX(50px); }
                .flip-backward { opacity: 0; transform: perspective(2000px) rotateY(10deg) translateX(-50px); }

                .book-spine {
                    position: absolute; top: 0; bottom: 0; left: 50%; width: 60px; margin-left: -30px;
                    background: linear-gradient(to right, rgba(0,0,0,0.6) 0%, rgba(0,0,0,0.9) 40%, rgba(30,20,10,1) 50%, rgba(0,0,0,0.9) 60%, rgba(0,0,0,0.6) 100%);
                    box-shadow: inset 0 0 20px rgba(0,0,0,1);
                    z-index: 10; pointer-events: none;
                }

                .book-page {
                    flex: 1; position: relative; z-index: 5;
                    overflow: auto; display: flex; flex-direction: column;
                }
                
                .left-page {
                    padding: 50px 60px 40px 50px; 
                    box-shadow: inset -20px 0 30px -10px rgba(0,0,0,0.2), inset 0 0 60px rgba(139,115,85,0.1);
                }
                
                .right-page {
                    padding: 50px 50px 40px 80px;
                    box-shadow: inset 20px 0 30px -10px rgba(0,0,0,0.2), inset 0 0 60px rgba(139,115,85,0.1);
                }
                
                .content-bg {
                    background: #e0ceae;
                    background-image: 
                        radial-gradient(rgba(139,115,85,0.08) 1px, transparent 1px), 
                        radial-gradient(rgba(139,115,85,0.08) 1px, transparent 1px),
                        linear-gradient(135deg, #f4ebd8 0%, #ebddc5 40%, #e0ceae 100%);
                    background-size: 20px 20px, 20px 20px, 100% 100%;
                    background-position: 0 0, 10px 10px, 0 0;
                }

                .page-watermark-left {
                    position: absolute; bottom: 20px; left: 30px;
                    font-family: 'Cinzel', serif; font-size: 16px; font-weight: 700; color: rgba(139,115,85,0.6);
                }
                .page-watermark-right {
                    position: absolute; bottom: 20px; right: 30px;
                    font-family: 'Cinzel', serif; font-size: 16px; font-weight: 700; color: rgba(139,115,85,0.6);
                }

                .dnd-markdown {
                    font-family: 'Inter', sans-serif;
                    color: #1a1510; font-size: 15px; line-height: 1.7;
                }
                .dnd-markdown > div > p:first-of-type::first-letter {
                    font-family: 'Cinzel', serif; font-size: 3.5em;
                    float: left; margin: 8px 8px 0 -4px;
                    line-height: 0.8; color: #5A1B1B;
                    text-shadow: 2px 2px 4px rgba(0,0,0,0.2);
                }
                
                .title-page-markdown > div > p:first-of-type::first-letter {
                    font-size: inherit; float: none; margin: 0; line-height: inherit; text-shadow: none;
                }

                .dnd-markdown p { margin-bottom: 16px; text-align: justify; }
                .dnd-markdown h1, .dnd-markdown h2, .dnd-markdown h3 {
                    font-family: 'Cinzel', serif; color: #5A1B1B; font-weight: 900;
                    margin-top: 24px; margin-bottom: 12px;
                    border-bottom: 1px solid rgba(207,185,145,0.6);
                    padding-bottom: 4px;
                }

                .dnd-markdown img {
                    max-width: 100%; border-radius: 4px; border: 8px solid #1a120b; outline: 2px solid #CFB991; outline-offset: -2px; box-shadow: inset 0 0 50px rgba(0,0,0,0.8), 0 0 20px rgba(0,0,0,0.5); margin: 16px auto; display: block;
                }

                .dnd-markdown blockquote {
                    background: linear-gradient(135deg, rgba(207,185,145,0.15), rgba(207,185,145,0.3));
                    border: none; border-radius: 4px;
                    padding: 16px 20px; margin: 20px 0;
                    box-shadow: inset 0 0 20px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.1);
                    position: relative;
                }
                .dnd-markdown blockquote::before {
                    content: ''; position: absolute; top: 0; left: 0; bottom: 0; width: 4px;
                    background: #5A1B1B; border-radius: 4px 0 0 4px;
                }
                .dnd-markdown ul, .dnd-markdown ol { margin-bottom: 16px; padding-left: 24px; }
                .dnd-markdown table {
                    width: 100%; border-collapse: separate; border-spacing: 0; margin: 16px 0;
                    font-size: 14px; border-radius: 6px; overflow: hidden;
                    box-shadow: 0 4px 10px rgba(0,0,0,0.08);
                }
                .dnd-markdown th, .dnd-markdown td {
                    border: 1px solid rgba(139,115,85,0.4); padding: 10px 14px; text-align: left;
                }
                .dnd-markdown th {
                    background: linear-gradient(to bottom, rgba(207,185,145,0.4), rgba(139,115,85,0.4));
                    font-family: 'Cinzel', serif; font-size: 13px; letter-spacing: 1px;
                    color: #5A1B1B; font-weight: 900; text-transform: uppercase;
                }
                .dnd-markdown td { background: rgba(255,255,255,0.4); }

                .audio-bar {
                    margin-top: 20px; padding: 12px 30px;
                    width: 100%; max-width: 1500px;
                    background: linear-gradient(to right, rgba(20,15,10,0.95), rgba(30,20,15,0.95), rgba(20,15,10,0.95));
                    border: 1px solid rgba(207,185,145,0.5); border-radius: 8px;
                    display: flex; align-items: center; justify-content: space-between;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.6), inset 0 0 15px rgba(207,185,145,0.1);
                    z-index: 50; position: relative;
                }
                .nav-group { display: flex; gap: 20px; align-items: center; }
                .nav-btn {
                    background: transparent; border: none;
                    color: #CFB991; font-family: 'Cinzel', serif; text-transform: uppercase;
                    font-weight: 900; font-size: 14px; letter-spacing: 1px;
                    cursor: pointer; transition: all 0.3s; outline: none; padding: 8px 16px;
                    border-radius: 4px;
                }
                .nav-btn:hover:not(.nav-btn--disabled) { color: #fff; background: rgba(207,185,145,0.1); }
                .nav-btn--disabled { opacity: 0.3; cursor: default; }
            `}} />
        </div>
    );
}
